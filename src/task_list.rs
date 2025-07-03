use serde::{Deserialize, Serialize};
use crate::task::Task;

#[derive(Debug, Clone, Serialize, Deserialize)] // 添加Clone
pub struct TaskList {
    pub name: String, 
    pub tasks: Vec<Task>,
    next_id: usize,
}

impl TaskList {
    pub fn new() -> Self {
        TaskList {
            name: "Task List".to_string(), 
            tasks: Vec::new(), 
            next_id: 1, 
        }
    }
    
    pub fn add_task(&mut self, description: String) -> usize{
        let id = self.next_id;
        self.tasks.push(Task::new(id, description));
        self.next_id += 1;
        id
    }
    
    // 先用bool返回值占位，后期换成自定义错误
    pub fn complete_task(&mut self, id: usize) -> bool {
        match self.tasks.iter_mut().find(|task| task.id() == id) { 
            Some(task) => { task.complete(); true }, 
            None => false, 
        }
    }
    
    pub fn suspend_task(&mut self, id: usize) -> bool {
        match self.tasks.iter_mut().find(|task| task.id() == id) {
            Some(task) => { 
                task.suspend();
                true
            },
            None => false,
        }
    }
    
    pub fn delete_task(&mut self, id: usize) -> bool {
        let index = self.tasks.iter().position(|task| task.id() == id);
        match index { 
            Some(i) => {
                self.tasks.remove(i);
                true
            }, 
            None => false
        }
    }

    // 获取所有任务
    pub fn get_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }
}
