use std::fmt;

// 自定义错误类型
#[derive(Debug)]
pub enum TodoError {
    IoError(std::io::Error),
    JsonError(serde_json::Error),
    TaskNotFound(usize),
    InvalidInput(String),
}

// 实现Display trait用于错误显示
impl fmt::Display for TodoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TodoError::IoError(err) => write!(f, "IO错误: {}", err),
            TodoError::JsonError(err) => write!(f, "JSON解析错误: {}", err),
            TodoError::TaskNotFound(id) => write!(f, "未找到ID为{}的任务", id),
            TodoError::InvalidInput(msg) => write!(f, "输入无效: {}", msg),
        }
    }
}

// 实现Error trait
impl std::error::Error for TodoError {}

// 从std::io::Error转换
impl From<std::io::Error> for TodoError {
    fn from(err: std::io::Error) -> Self {
        TodoError::IoError(err)
    }
}

// 从serde_json::Error转换
impl From<serde_json::Error> for TodoError {
    fn from(err: serde_json::Error) -> Self {
        TodoError::JsonError(err)
    }
}

pub type Result<T> = std::result::Result<T, TodoError>;