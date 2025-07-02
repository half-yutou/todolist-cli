use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Pending,
    Suspended, 
    Completed, 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    id: usize, 
    description: String, 
    status: TaskStatus, 
    // ISO 8601: 2025-07-02T16:05:25
    created_at: String, 
    completed_at: Option<String>, 
}

impl Task {
    pub fn new(id: usize, description: String) -> Self {
        let created_at = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        Task {
            id, 
            description, 
            status: TaskStatus::Pending, 
            created_at, 
            completed_at: None
        }
    }
    
    pub fn suspend(&mut self) {
        self.status = TaskStatus::Suspended;
    }
    
    pub fn complete(&mut self) {
        self.status = TaskStatus::Completed;
    }
    
    pub fn is_completed(&self) -> bool {
        self.status == TaskStatus::Completed
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let status_symbol = match self.status {
            TaskStatus::Pending   => "[ ]pending  ",
            TaskStatus::Suspended => "[.]suspended", 
            TaskStatus::Completed => "[✓]completed",
        };
        write!(f, "{} {} - {}", status_symbol, self.id, self.description)
    }
}


