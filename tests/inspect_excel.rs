use calamine::{open_workbook, Reader, Xlsx};
use std::path::Path;

#[test]
fn inspect_excel_headers() {
    let path = Path::new("Project_Schedule_Grouped_By_Workstream_ additional activities.xlsx");
    let mut workbook: Xlsx<_> = open_workbook(path).expect("Cannot open file");
    
    if let Some(Ok(range)) = workbook.worksheet_range_at(0) {
        if let Some(row) = range.rows().next() {
            println!("EXCEL HEADERS: {:?}", row);
        }
    }
}
