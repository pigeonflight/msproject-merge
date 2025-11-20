use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "Project")]
pub struct Project {
    #[serde(rename = "Title", default)]
    pub title: String,
    #[serde(rename = "Tasks")]
    pub tasks: Tasks,
    // Add other project-level fields as needed
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Tasks {
    #[serde(rename = "Task", default)]
    pub task: Vec<MspdiTask>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MspdiTask {
    #[serde(rename = "UID")]
    pub uid: i32,
    #[serde(rename = "ID")]
    pub id: i32,
    #[serde(rename = "Name", default)]
    pub name: String,
    #[serde(rename = "Start", default)]
    pub start: String,
    #[serde(rename = "Finish", default)]
    pub finish: String,
    #[serde(rename = "Duration", default)]
    pub duration: String,
    #[serde(rename = "PercentComplete", default)]
    pub percent_complete: i32,
    #[serde(rename = "Active", default)]
    pub active: i32,
    #[serde(rename = "Manual", default)]
    pub manual: i32,
    #[serde(rename = "OutlineNumber", default)]
    pub outline_number: String,
    #[serde(rename = "OutlineLevel", default)]
    pub outline_level: i32,
    #[serde(rename = "Priority", default)]
    pub priority: i32,
    #[serde(rename = "Notes", default)]
    pub notes: String,
    
    // Extended fields often found in MSPDI
    #[serde(rename = "WBS", default)]
    pub wbs: String,

    #[serde(rename = "PredecessorLink", default)]
    pub predecessor_link: Vec<MspdiPredecessorLink>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MspdiPredecessorLink {
    #[serde(rename = "PredecessorUID")]
    pub predecessor_uid: i32,
    #[serde(rename = "Type", default)]
    pub link_type: i32,
    #[serde(rename = "CrossProject", default)]
    pub cross_project: i32,
    #[serde(rename = "LinkLag", default)]
    pub link_lag: i32,
    #[serde(rename = "LagFormat", default)]
    pub lag_format: i32,
}

// Helper to create a default Project
impl Default for Project {
    fn default() -> Self {
        Self {
            title: String::new(),
            tasks: Tasks { task: Vec::new() },
        }
    }
}
