use crate::task::Task;
use std::collections::HashMap;

pub fn merge_projects(base: &mut Vec<Task>, overlay: &[Task]) {
    // Create a map of WBS -> Index for the base project
    let mut wbs_map: HashMap<String, usize> = HashMap::new();
    for (idx, task) in base.iter().enumerate() {
        if !task.wbs.is_empty() {
            wbs_map.insert(task.wbs.clone(), idx);
        }
    }

    for overlay_task in overlay {
        if !overlay_task.wbs.is_empty() {
            if let Some(&idx) = wbs_map.get(&overlay_task.wbs) {
                // Update existing task
                let base_task = &mut base[idx];
                
                // Update fields if they are present in overlay (and not default/empty)
                // Note: This logic assumes overlay has newer/better data
                
                // Always update status and percent complete
                base_task.percent_complete = overlay_task.percent_complete;
                base_task.status = overlay_task.status;
                
                // Update dates
                base_task.start_date = overlay_task.start_date;
                base_task.end_date = overlay_task.end_date;
                base_task.duration_days = overlay_task.duration_days;
                
                // Update other fields
                if !overlay_task.assignee.is_empty() {
                    base_task.assignee = overlay_task.assignee.clone();
                }
                if !overlay_task.description.is_empty() {
                    base_task.description = overlay_task.description.clone();
                }
                
                // Optional: Update name? Usually WBS implies same task, but name might be refined
                // base_task.name = overlay_task.name.clone();
                
            } else {
                // New task - append it
                // Note: Inserting into the correct hierarchy position is hard without a full tree structure.
                // For now, we just append it.
                base.push(overlay_task.clone());
            }
        } else {
            // Overlay task has no WBS. 
            // Strategy: Try to match by name? Or just append?
            // For now, append if name doesn't exist? 
            // Let's just append to be safe, or maybe ignore?
            // User requirement was WBS-based. Let's ignore for now to avoid duplicates, 
            // or append if it looks important.
            // Let's append.
            base.push(overlay_task.clone());
        }
    }
    
    // Re-sort by WBS if possible?
    // base.sort_by(|a, b| a.wbs.cmp(&b.wbs)); // Simple string sort might be wrong for 1.10 vs 1.2
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task::{Task, TaskStatus};

    #[test]
    fn test_merge_update_existing() {
        let mut base_task = Task::new("Base Task".to_string());
        base_task.wbs = "1.1".to_string();
        base_task.percent_complete = 0;
        
        let mut overlay_task = Task::new("Overlay Task".to_string());
        overlay_task.wbs = "1.1".to_string();
        overlay_task.percent_complete = 50;
        overlay_task.status = TaskStatus::InProgress;
        
        let mut base = vec![base_task];
        let overlay = vec![overlay_task];
        
        merge_projects(&mut base, &overlay);
        
        assert_eq!(base.len(), 1);
        assert_eq!(base[0].percent_complete, 50);
        assert_eq!(base[0].status, TaskStatus::InProgress);
    }

    #[test]
    fn test_merge_append_new() {
        let mut base_task = Task::new("Base Task".to_string());
        base_task.wbs = "1.1".to_string();
        
        let mut overlay_task = Task::new("New Task".to_string());
        overlay_task.wbs = "1.2".to_string();
        
        let mut base = vec![base_task];
        let overlay = vec![overlay_task];
        
        merge_projects(&mut base, &overlay);
        
        assert_eq!(base.len(), 2);
        assert_eq!(base[1].wbs, "1.2");
        assert_eq!(base[1].name, "New Task");
    }
}
