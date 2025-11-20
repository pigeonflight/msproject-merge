use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub name: String,
    pub description: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub status: TaskStatus,
    pub priority: u32,
    pub assignee: String,
    pub duration_days: u32,
    pub percent_complete: u32,
    pub source_file: usize,
    pub wbs: String,
    pub predecessors: Vec<Predecessor>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Predecessor {
    pub predecessor_uid: i32,
    pub link_type: i32,
    pub link_lag: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    NotStarted,
    InProgress,
    Completed,
    OnHold,
    Cancelled,
}

impl Ord for TaskStatus {
    fn cmp(&self, other: &Self) -> Ordering {
        use TaskStatus::*;
        let order = |s: &TaskStatus| match s {
            NotStarted => 0,
            InProgress => 1,
            OnHold => 2,
            Completed => 3,
            Cancelled => 4,
        };
        order(self).cmp(&order(other))
    }
}

impl PartialOrd for TaskStatus {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Task {
    pub fn new(name: String) -> Self {
        let today = chrono::Local::now().date_naive();
        Self {
            name,
            description: String::new(),
            start_date: today,
            end_date: today,
            status: TaskStatus::NotStarted,
            priority: 0,
            assignee: String::new(),
            duration_days: 0,
            percent_complete: 0,
            source_file: 0,
            wbs: String::new(),
            predecessors: Vec::new(),
        }
    }
}


