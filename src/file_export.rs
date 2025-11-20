use rust_xlsxwriter::{Workbook, Format, FormatAlign};
use crate::task::Task;

pub fn export_to_xlsx(tasks: &[Task], path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    
    // Set up header format
    let header_format = Format::new()
        .set_bold()
        .set_align(FormatAlign::Center);
    
    // Write headers
    worksheet.write_string_with_format(0, 0, "Task Name", &header_format)?;
    worksheet.write_string_with_format(0, 1, "Description", &header_format)?;
    worksheet.write_string_with_format(0, 2, "Start Date", &header_format)?;
    worksheet.write_string_with_format(0, 3, "End Date", &header_format)?;
    worksheet.write_string_with_format(0, 4, "Status", &header_format)?;
    worksheet.write_string_with_format(0, 5, "Priority", &header_format)?;
    worksheet.write_string_with_format(0, 6, "Assignee", &header_format)?;
    worksheet.write_string_with_format(0, 7, "Duration (Days)", &header_format)?;
    worksheet.write_string_with_format(0, 8, "% Complete", &header_format)?;
    
    // Write task data
    for (row, task) in tasks.iter().enumerate() {
        let row_num = (row + 1) as u32;
        
        worksheet.write_string(row_num, 0, &task.name)?;
        worksheet.write_string(row_num, 1, &task.description)?;
        
        // Write dates as formatted strings
        worksheet.write_string(row_num, 2, &task.start_date.format("%Y-%m-%d").to_string())?;
        worksheet.write_string(row_num, 3, &task.end_date.format("%Y-%m-%d").to_string())?;
        worksheet.write_string(row_num, 4, format!("{:?}", task.status))?;
        worksheet.write_number(row_num, 5, task.priority as f64)?;
        worksheet.write_string(row_num, 6, &task.assignee)?;
        worksheet.write_number(row_num, 7, task.duration_days as f64)?;
        worksheet.write_number(row_num, 8, task.percent_complete as f64)?;
    }
    
    // Auto-fit columns
    worksheet.set_column_width(0, 30.0)?;
    worksheet.set_column_width(1, 40.0)?;
    worksheet.set_column_width(2, 12.0)?;
    worksheet.set_column_width(3, 12.0)?;
    worksheet.set_column_width(4, 15.0)?;
    worksheet.set_column_width(5, 10.0)?;
    worksheet.set_column_width(6, 20.0)?;
    worksheet.set_column_width(7, 15.0)?;
    worksheet.set_column_width(8, 12.0)?;
    
    workbook.save(path)?;
    Ok(())
}

