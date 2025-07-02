mod task;
mod task_list;
mod storage;
mod error;

use std::io::{self, Write};
use storage::Storage;
use task_list::TaskList;
use error::Result;

fn main() {
    if let Err(e) = run() {
        eprintln!("错误: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    println!("=== Todo List CLI ===");
    
    // 加载或创建任务列表
    let mut task_list = match Storage::load_tasks() {
        Ok(tasks) => {
            if Storage::storage_exists() {
                println!("✅ 成功读取 tasks.json 文件");
            } else {
                println!("📝 tasks.json 不存在，已创建新的任务列表");
                // 保存空的任务列表到文件
                Storage::save_tasks(&tasks)?;
            }
            tasks
        }
        Err(e) => {
            eprintln!("❌ 读取任务文件失败: {}", e);
            println!("📝 创建新的任务列表");
            let new_list = TaskList::new();
            Storage::save_tasks(&new_list)?;
            new_list
        }
    };
    
    // 显示当前任务列表
    display_tasks(&task_list);
    
    // 主循环
    loop {
        display_menu();
        
        let choice = get_user_input("请选择操作 (0-4): ")?;
        
        match choice.trim() {
            "0" => {
                println!("👋 再见！");
                break;
            }
            "1" => {
                add_task(&mut task_list)?;
            }
            "2" => {
                suspend_task(&mut task_list)?;
            }
            "3" => {
                complete_task(&mut task_list)?;
            }
            "4" => {
                delete_task(&mut task_list)?;
            }
            _ => {
                println!("❌ 无效选择，请输入 0-4 之间的数字");
                continue;
            }
        }
        
        // 保存任务列表
        Storage::save_tasks(&task_list)?;
        
        // 显示更新后的任务列表
        println!();
        display_tasks(&task_list);
    }
    
    Ok(())
}

fn display_menu() {
    println!("\n=== 操作菜单 ===");
    println!("0. 退出程序");
    println!("1. 添加任务");
    println!("2. 挂起任务");
    println!("3. 完成任务");
    println!("4. 删除任务");
}

fn display_tasks(task_list: &TaskList) {
    println!("\n📋 当前任务列表:");
    let tasks = task_list.get_tasks();
    
    if tasks.is_empty() {
        println!("  (暂无任务)");
    } else {
        for task in tasks {
            println!("  {}", task);
        }
    }
    
    let pending_count = tasks.iter().filter(|t| !t.is_completed()).count();
    println!("📊 待处理任务: {} 个", pending_count);
}

fn get_user_input(prompt: &str) -> Result<String> {
    print!("{}", prompt);
    io::stdout().flush().map_err(|e| error::TodoError::IoError(e))?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|e| error::TodoError::IoError(e))?;
    
    Ok(input.trim().to_string())
}

fn add_task(task_list: &mut TaskList) -> Result<()> {
    let description = get_user_input("请输入任务描述: ")?;
    
    if description.is_empty() {
        println!("❌ 任务描述不能为空");
        return Ok(());
    }
    
    let id = task_list.add_task(description.clone());
    println!("✅ 已添加任务 #{}: {}", id, description);
    
    Ok(())
}

fn suspend_task(task_list: &mut TaskList) -> Result<()> {
    let id_str = get_user_input("请输入要挂起的任务ID: ")?;
    
    let id: usize = match id_str.parse() {
        Ok(id) => id,
        Err(_) => {
            println!("❌ 请输入有效的数字ID");
            return Ok(());
        }
    };
    
    if task_list.suspend_task(id) {
        println!("⏸️  任务 #{} 已挂起", id);
    } else {
        println!("❌ 未找到ID为 {} 的任务", id);
    }
    
    Ok(())
}

fn complete_task(task_list: &mut TaskList) -> Result<()> {
    let id_str = get_user_input("请输入要完成的任务ID: ")?;
    
    let id: usize = match id_str.parse() {
        Ok(id) => id,
        Err(_) => {
            println!("❌ 请输入有效的数字ID");
            return Ok(());
        }
    };
    
    if task_list.complete_task(id) {
        println!("✅ 任务 #{} 已完成！", id);
    } else {
        println!("❌ 未找到ID为 {} 的任务", id);
    }
    
    Ok(())
}

fn delete_task(task_list: &mut TaskList) -> Result<()> {
    let id_str = get_user_input("请输入要删除的任务ID: ")?;
    
    let id: usize = match id_str.parse() {
        Ok(id) => id,
        Err(_) => {
            println!("❌ 请输入有效的数字ID");
            return Ok(());
        }
    };
    
    if task_list.delete_task(id) {
        println!("🗑️  任务 #{} 已删除", id);
    } else {
        println!("❌ 未找到ID为 {} 的任务", id);
    }
    
    Ok(())
}
