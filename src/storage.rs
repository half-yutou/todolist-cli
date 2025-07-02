use crate::error::Result;
use crate::task_list::TaskList;
use std::fs;
use std::path::Path;

const STORAGE_FILE: &str = "tasks.json";

pub struct Storage;

impl Storage {
    
    // 保存任务到文件
    pub fn save_tasks(task_list: &TaskList) -> Result<()> {
        let json = serde_json::to_string_pretty(task_list)?;
        fs::write(STORAGE_FILE, json)?;
        Ok(())
    }

    // 从文件加载任务
    pub fn load_tasks() -> Result<TaskList> {
        if !Path::new(STORAGE_FILE).exists() {
            return Ok(TaskList::new());
        }

        let content = fs::read_to_string(STORAGE_FILE)?;
        let task_list: TaskList = serde_json::from_str(&content)?;
        Ok(task_list)
    }

    // 检查存储文件是否存在
    pub fn storage_exists() -> bool {
        Path::new(STORAGE_FILE).exists()
    }
}