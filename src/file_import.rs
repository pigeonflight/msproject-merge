use calamine::{open_workbook, Reader, Xlsx};
use std::path::Path;
use crate::task::{Task, TaskStatus};

pub fn import_xlsx(path: &Path) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    let mut workbook: Xlsx<_> = open_workbook(path)?;
    let mut tasks = Vec::new();

    // Try to read from the first sheet
    if let Some(Ok(range)) = workbook.worksheet_range_at(0) {
        let mut rows = range.rows();
        
        // Skip header row if present
        if let Some(header) = rows.next() {
            // Try to detect column indices
            let mut name_col = None;
            let mut desc_col = None;
            let mut start_col = None;
            let mut end_col = None;
            let mut status_col = None;
            let mut priority_col = None;
            let mut assignee_col = None;
            let mut duration_col = None;
            let mut percent_col = None;

            let mut wbs_col = None;

            for (idx, cell) in header.iter().enumerate() {
                let cell_str = cell.to_string().to_lowercase();
                if cell_str.contains("task") && cell_str.contains("name") || cell_str == "name" {
                    name_col = Some(idx);
                } else if cell_str.contains("description") || cell_str == "description" {
                    desc_col = Some(idx);
                } else if cell_str.contains("start") && cell_str.contains("date") || cell_str == "start" {
                    start_col = Some(idx);
                } else if cell_str.contains("end") && cell_str.contains("date") || cell_str.contains("finish") {
                    end_col = Some(idx);
                } else if cell_str.contains("status") {
                    status_col = Some(idx);
                } else if cell_str.contains("priority") {
                    priority_col = Some(idx);
                } else if cell_str.contains("assignee") || cell_str.contains("resource") {
                    assignee_col = Some(idx);
                } else if cell_str.contains("duration") {
                    duration_col = Some(idx);
                } else if cell_str.contains("percent") || cell_str.contains("%") {
                    percent_col = Some(idx);
                } else if cell_str == "wbs" {
                    wbs_col = Some(idx);
                }
            }

            // Read data rows
            for row in rows {
                let get_cell = |col: Option<usize>| {
                    col.and_then(|c| row.get(c)).map(|c| c.to_string())
                };

                let name = get_cell(name_col)
                    .filter(|s| !s.is_empty())
                    .unwrap_or_else(|| String::from("Unnamed Task"));

                if name == "Unnamed Task" && row.iter().all(|c| c.to_string().is_empty()) {
                    continue; // Skip empty rows
                }

                let description = get_cell(desc_col).unwrap_or_default();
                let assignee = get_cell(assignee_col).unwrap_or_default();
                let wbs = get_cell(wbs_col).unwrap_or_default();
                
                // Parse dates
                let today = chrono::Local::now().date_naive();
                let start_date = get_cell(start_col)
                    .and_then(|s| parse_date(&s))
                    .unwrap_or(today);
                let end_date = get_cell(end_col)
                    .and_then(|s| parse_date(&s))
                    .unwrap_or(start_date);

                // Parse status
                let status = get_cell(status_col)
                    .and_then(|s| parse_status(&s))
                    .unwrap_or(TaskStatus::NotStarted);

                // Parse priority
                let priority = get_cell(priority_col)
                    .and_then(|s| s.parse::<u32>().ok())
                    .unwrap_or(0);

                // Parse duration
                let duration_days = get_cell(duration_col)
                    .and_then(|s| parse_duration(&s))
                    .unwrap_or(0);

                // Parse percent complete
                let percent_complete = get_cell(percent_col)
                    .and_then(|s| {
                        s.replace('%', "").trim().parse::<u32>().ok()
                    })
                    .unwrap_or(0);

                tasks.push(Task {
                    name,
                    description,
                    start_date,
                    end_date,
                    status,
                    priority,
                    assignee,
                    duration_days,
                    percent_complete,
                    source_file: 0, // Will be set by caller
                    wbs,
                    predecessors: Vec::new(),
                });
            }
        }
    }

    Ok(tasks)
}

pub fn import_mpp(path: &Path) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    // Note: Direct MPP file reading in Rust is limited.
    // This is a placeholder that attempts to handle MPP files.
    // For production use, you may need to:
    // 1. Convert MPP to XLSX/XML first using external tools
    // 2. Use COM interop on Windows (not cross-platform)
    // 3. Use a Python bridge with python-pptx or similar
    
    // For now, we'll try to read it as if it might be XML-based
    // or return an error suggesting conversion
    
    let content = std::fs::read(path)?;
    
    // Try to detect if it's actually an XML file (some MPP exports are XML)
    if content.starts_with(b"<?xml") || content.starts_with(b"<") {
        // Try parsing as XML
        return import_mpp_xml(&content);
    }
    
    // If it's a binary MPP file, we can't easily read it
    Err(format!(
        "Binary MPP files are not directly supported. Please export your MPP file to XLSX format first.\n\
         File: {}",
        path.display()
    ).into())
}

use quick_xml::de::from_str;
use crate::mspdi::Project;

fn import_mpp_xml(content: &[u8]) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    let xml_str = String::from_utf8_lossy(content);
    
    // Parse XML into MSPDI struct
    let project: Project = from_str(&xml_str)?;
    
    let mut tasks = Vec::new();
    
    for mspdi_task in project.tasks.task {
        // Skip summary tasks or empty tasks if needed, or handle them
        // For now, we import everything that has a name
        if mspdi_task.name.is_empty() {
            continue;
        }

        let mut task = Task::new(mspdi_task.name);
        
        // Parse dates
        if let Some(date) = parse_mspdi_date(&mspdi_task.start) {
            task.start_date = date;
        }
        if let Some(date) = parse_mspdi_date(&mspdi_task.finish) {
            task.end_date = date;
        }
        
        // Parse duration (MSPDI format is usually PT8H0M0S or similar ISO8601 duration)
        // But MS Project XML often uses "PT8H0M0S"
        task.duration_days = parse_mspdi_duration(&mspdi_task.duration);
        
        task.percent_complete = mspdi_task.percent_complete as u32;
        task.priority = mspdi_task.priority as u32;
        task.description = mspdi_task.notes;
        task.wbs = mspdi_task.wbs;
        
        // Import predecessors
        for pred in mspdi_task.predecessor_link {
            task.predecessors.push(crate::task::Predecessor {
                predecessor_uid: pred.predecessor_uid,
                link_type: pred.link_type,
                link_lag: pred.link_lag,
            });
        }
        
        // Status logic based on percent complete
        if task.percent_complete == 100 {
            task.status = TaskStatus::Completed;
        } else if task.percent_complete > 0 {
            task.status = TaskStatus::InProgress;
        } else {
            task.status = TaskStatus::NotStarted;
        }

        tasks.push(task);
    }
    
    Ok(tasks)
}

fn parse_mspdi_date(s: &str) -> Option<chrono::NaiveDate> {
    // MSPDI dates are usually ISO8601: YYYY-MM-DDTHH:MM:SS
    if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S") {
        return Some(dt.date());
    }
    // Fallback to date only
    if let Ok(d) = chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d") {
        return Some(d);
    }
    None
}

fn parse_mspdi_duration(s: &str) -> u32 {
    // Format is usually PT<hours>H<minutes>M<seconds>S
    // We want days. Standard day is 8 hours.
    // This is a simplified parser.
    let s = s.trim_start_matches("PT");
    let mut hours = 0;
    
    if let Some(h_idx) = s.find('H') {
        if let Ok(h) = s[..h_idx].parse::<u32>() {
            hours = h;
        }
    }
    
    // Rough conversion: 8 hours = 1 day
    if hours > 0 {
        return (hours + 7) / 8; // Ceiling division
    }
    
    0
}

// Keep existing helper functions if needed, or remove if unused
fn parse_date(s: &str) -> Option<chrono::NaiveDate> {
    // ... (keep existing implementation if needed for XLSX)
    // Try various date formats
    let formats = [
        "%Y-%m-%d",
        "%m/%d/%Y",
        "%d/%m/%Y",
        "%Y/%m/%d",
        "%d-%m-%Y",
    ];
    
    for format in &formats {
        if let Ok(date) = chrono::NaiveDate::parse_from_str(s.trim(), format) {
            return Some(date);
        }
    }
    
    // Try Excel serial date (days since 1900-01-01)
    if let Ok(days) = s.parse::<f64>() {
        let base = chrono::NaiveDate::from_ymd_opt(1899, 12, 30)?;
        return base.checked_add_signed(chrono::Duration::days(days as i64));
    }
    
    None
}

fn parse_status(s: &str) -> Option<TaskStatus> {
     let s_lower = s.to_lowercase();
    if s_lower.contains("not") && s_lower.contains("start") {
        Some(TaskStatus::NotStarted)
    } else if s_lower.contains("in") && s_lower.contains("progress") {
        Some(TaskStatus::InProgress)
    } else if s_lower.contains("complete") {
        Some(TaskStatus::Completed)
    } else if s_lower.contains("hold") {
        Some(TaskStatus::OnHold)
    } else if s_lower.contains("cancel") {
        Some(TaskStatus::Cancelled)
    } else {
        None
    }
}

fn parse_duration(s: &str) -> Option<u32> {
    // Try to parse duration strings like "5d", "5 days", "5", etc.
    let s = s.trim().to_lowercase();
    if let Some(num_str) = s.split_whitespace().next() {
        if let Ok(num) = num_str.parse::<u32>() {
            return Some(num);
        }
    }
    s.parse::<u32>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_import_mpp_xml() {
        let xml = r#"
        <Project xmlns="http://schemas.microsoft.com/project">
            <Tasks>
                <Task>
                    <UID>1</UID>
                    <ID>1</ID>
                    <Name>Task 1</Name>
                    <Start>2023-01-01T08:00:00</Start>
                    <Finish>2023-01-05T17:00:00</Finish>
                    <Duration>PT32H0M0S</Duration>
                    <PercentComplete>50</PercentComplete>
                    <Active>1</Active>
                    <Manual>0</Manual>
                    <OutlineNumber>1</OutlineNumber>
                    <OutlineLevel>1</OutlineLevel>
                    <Priority>500</Priority>
                    <Notes>Test Note</Notes>
                </Task>
            </Tasks>
        </Project>
        "#;

        let tasks = import_mpp_xml(xml.as_bytes()).expect("Failed to parse XML");
        
        assert_eq!(tasks.len(), 1);
        let task = &tasks[0];
        assert_eq!(task.name, "Task 1");
        assert_eq!(task.start_date, chrono::NaiveDate::from_ymd_opt(2023, 1, 1).unwrap());
        assert_eq!(task.end_date, chrono::NaiveDate::from_ymd_opt(2023, 1, 5).unwrap());
        assert_eq!(task.duration_days, 4); // 32 hours / 8 hours per day
        assert_eq!(task.percent_complete, 50);
        assert_eq!(task.status, TaskStatus::InProgress);
        assert_eq!(task.description, "Test Note");
    }

    #[test]
    fn test_import_mpp_xml_with_predecessors() {
        let xml = r#"
        <Project xmlns="http://schemas.microsoft.com/project">
            <Tasks>
                <Task>
                    <UID>1</UID>
                    <ID>1</ID>
                    <Name>Task 1</Name>
                    <WBS>1</WBS>
                </Task>
                <Task>
                    <UID>2</UID>
                    <ID>2</ID>
                    <Name>Task 2</Name>
                    <WBS>2</WBS>
                    <PredecessorLink>
                        <PredecessorUID>1</PredecessorUID>
                        <Type>1</Type>
                        <CrossProject>0</CrossProject>
                        <LinkLag>0</LinkLag>
                        <LagFormat>7</LagFormat>
                    </PredecessorLink>
                </Task>
            </Tasks>
        </Project>
        "#;

        let tasks = import_mpp_xml(xml.as_bytes()).unwrap();
        assert_eq!(tasks.len(), 2);
        assert_eq!(tasks[1].predecessors.len(), 1);
        assert_eq!(tasks[1].predecessors[0].predecessor_uid, 1);
        assert_eq!(tasks[1].predecessors[0].link_type, 1);
    }
}


