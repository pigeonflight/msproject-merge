use quick_xml::se::to_string;
use crate::task::Task;
use crate::mspdi::{Project, Tasks, MspdiTask};
use std::fs::File;
use std::io::Write;

pub fn export_to_xml(tasks: &[Task], path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut mspdi_tasks = Vec::new();
    
    for (idx, task) in tasks.iter().enumerate() {
        let mspdi_task = MspdiTask {
            uid: (idx + 1) as i32,
            id: (idx + 1) as i32,
            name: task.name.clone(),
            start: task.start_date.format("%Y-%m-%dT08:00:00").to_string(),
            finish: task.end_date.format("%Y-%m-%dT17:00:00").to_string(),
            duration: format!("PT{}H0M0S", task.duration_days * 8),
            percent_complete: task.percent_complete as i32,
            active: 1,
            manual: 0,
            outline_number: task.wbs.clone(), // Use WBS as outline number for now
            outline_level: 1,
            priority: task.priority as i32,
            notes: task.description.clone(),
            wbs: task.wbs.clone(),
            predecessor_link: task.predecessors.iter().map(|p| crate::mspdi::MspdiPredecessorLink {
                predecessor_uid: p.predecessor_uid,
                link_type: p.link_type,
                cross_project: 0,
                link_lag: p.link_lag,
                lag_format: 7, // Default format
            }).collect(),
        };
        mspdi_tasks.push(mspdi_task);
    }
    
    let project = Project {
        title: "Merged Project".to_string(),
        tasks: Tasks { task: mspdi_tasks },
    };
    
    let xml_string = to_string(&project)?;
    
    // Add XML declaration
    let final_xml = format!("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n{}", xml_string);
    
    let mut file = File::create(path)?;
    file.write_all(final_xml.as_bytes())?;
    
    Ok(())
}
