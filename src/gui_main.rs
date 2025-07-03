mod task;
mod task_list;
mod storage;
mod error;

use eframe::egui;
use storage::Storage;
use task::Task;
use task_list::TaskList;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "📋 Todo List GUI",
        options,
        Box::new(|cc| {
            // 配置中文字体支持
            setup_custom_fonts(&cc.egui_ctx);
            Box::new(TodoApp::new())
        }),
    )
}

// 添加字体配置函数
fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    
    // 尝试加载系统中文字体
    let mut font_loaded = false;
    
    #[cfg(target_os = "macos")]
    {
        // macOS 字体路径列表
        let font_paths = [
            "/System/Library/Fonts/PingFang.ttc",
            "/System/Library/Fonts/Helvetica.ttc",
            "/System/Library/Fonts/Arial Unicode.ttf",
            "/Library/Fonts/Arial Unicode.ttf",
            "/System/Library/Fonts/Apple Color Emoji.ttc",
        ];
        
        for (i, path) in font_paths.iter().enumerate() {
            if let Ok(font_data) = std::fs::read(path) {
                let font_name = format!("system_font_{}", i);
                fonts.font_data.insert(
                    font_name.clone(),
                    egui::FontData::from_owned(font_data),
                );
                
                // 添加到字体族
                fonts.families.get_mut(&egui::FontFamily::Proportional)
                    .unwrap()
                    .insert(0, font_name.clone());
                
                fonts.families.get_mut(&egui::FontFamily::Monospace)
                    .unwrap()
                    .insert(0, font_name);
                
                font_loaded = true;
            }
        }
    }
    
    #[cfg(target_os = "windows")]
    {
        let font_paths = [
            "C:/Windows/Fonts/msyh.ttc",      // 微软雅黑
            "C:/Windows/Fonts/simsun.ttc",    // 宋体
            "C:/Windows/Fonts/arial.ttf",     // Arial
            "C:/Windows/Fonts/seguiemj.ttf",  // Segoe UI Emoji
        ];
        
        for (i, path) in font_paths.iter().enumerate() {
            if let Ok(font_data) = std::fs::read(path) {
                let font_name = format!("system_font_{}", i);
                fonts.font_data.insert(
                    font_name.clone(),
                    egui::FontData::from_owned(font_data),
                );
                
                fonts.families.get_mut(&egui::FontFamily::Proportional)
                    .unwrap()
                    .insert(0, font_name.clone());
                
                fonts.families.get_mut(&egui::FontFamily::Monospace)
                    .unwrap()
                    .insert(0, font_name);
                
                font_loaded = true;
            }
        }
    }
    
    #[cfg(target_os = "linux")]
    {
        let font_paths = [
            "/usr/share/fonts/truetype/wqy/wqy-microhei.ttc",
            "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
            "/usr/share/fonts/TTF/DejaVuSans.ttf",
            "/usr/share/fonts/truetype/noto/NotoSansCJK-Regular.ttc",
            "/usr/share/fonts/opentype/noto/NotoColorEmoji.ttf",
        ];
        
        for (i, path) in font_paths.iter().enumerate() {
            if let Ok(font_data) = std::fs::read(path) {
                let font_name = format!("system_font_{}", i);
                fonts.font_data.insert(
                    font_name.clone(),
                    egui::FontData::from_owned(font_data),
                );
                
                fonts.families.get_mut(&egui::FontFamily::Proportional)
                    .unwrap()
                    .insert(0, font_name.clone());
                
                fonts.families.get_mut(&egui::FontFamily::Monospace)
                    .unwrap()
                    .insert(0, font_name);
                
                font_loaded = true;
            }
        }
    }
    
    if !font_loaded {
        println!("警告: 未能加载系统字体，中文可能无法正常显示");
    }
    
    // 应用字体配置
    ctx.set_fonts(fonts);
}

struct TodoApp {
    task_list: TaskList,
    new_task_description: String,
    selected_task_id: Option<usize>,
    status_message: String,
    show_completed: bool,
}

impl TodoApp {
    fn new() -> Self {
        let task_list = match Storage::load_tasks() {
            Ok(tasks) => {
                if !Storage::storage_exists() {
                    // 如果文件不存在，保存空列表
                    let _ = Storage::save_tasks(&tasks);
                }
                tasks
            }
            Err(_) => {
                let new_list = TaskList::new();
                let _ = Storage::save_tasks(&new_list);
                new_list
            }
        };

        Self {
            task_list,
            new_task_description: String::new(),
            selected_task_id: None,
            status_message: "就绪".to_string(),
            show_completed: true,
        }
    }

    fn save_tasks(&mut self) {
        match Storage::save_tasks(&self.task_list) {
            Ok(_) => self.status_message = "✅ 已保存".to_string(),
            Err(e) => self.status_message = format!("❌ 保存失败: {:?}", e),
        }
    }

    fn add_task(&mut self) {
        if !self.new_task_description.trim().is_empty() {
            let id = self.task_list.add_task(self.new_task_description.clone());
            self.status_message = format!("✅ 已添加任务 #{}", id);
            self.new_task_description.clear();
            self.save_tasks();
        } else {
            self.status_message = "❌ 任务描述不能为空".to_string();
        }
    }

    fn complete_task(&mut self, id: usize) {
        if self.task_list.complete_task(id) {
            self.status_message = format!("✅ 任务 #{} 已完成", id);
            self.save_tasks();
        } else {
            self.status_message = format!("❌ 未找到任务 #{}", id);
        }
    }

    fn suspend_task(&mut self, id: usize) {
        if self.task_list.suspend_task(id) {
            self.status_message = format!("⏸️ 任务 #{} 已挂起", id);
            self.save_tasks();
        } else {
            self.status_message = format!("❌ 未找到任务 #{}", id);
        }
    }

    fn delete_task(&mut self, id: usize) {
        if self.task_list.delete_task(id) {
            self.status_message = format!("🗑️ 任务 #{} 已删除", id);
            self.save_tasks();
        } else {
            self.status_message = format!("❌ 未找到任务 #{}", id);
        }
    }

    fn get_status_color(&self, task: &Task) -> egui::Color32 {
        match task.is_completed() {
            true => egui::Color32::from_rgb(100, 200, 100), // 绿色
            false => egui::Color32::from_rgb(200, 200, 100), // 黄色
        }
    }

    fn get_status_text(&self, task: &Task) -> &str {
        if task.is_completed() {
            "✅ 已完成"
        } else {
            "⏳ 进行中"
        }
    }
}

impl eframe::App for TodoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // 标题
            ui.heading("📋 Todo List GUI");
            ui.separator();

            // 添加任务区域
            ui.horizontal(|ui| {
                ui.label("新任务:");
                let response = ui.text_edit_singleline(&mut self.new_task_description);
                
                // 支持回车键添加任务
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.add_task();
                }
                
                if ui.button("➕ 添加").clicked() {
                    self.add_task();
                }
            });

            ui.separator();

            // 过滤选项
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.show_completed, "显示已完成任务");
                
                let tasks = self.task_list.get_tasks();
                let total_count = tasks.len();
                let pending_count = tasks.iter().filter(|t| !t.is_completed()).count();
                let completed_count = total_count - pending_count;
                
                ui.label(format!("📊 总计: {} | 待处理: {} | 已完成: {}", 
                    total_count, pending_count, completed_count));
            });

            ui.separator();

            // 任务列表 - 修复借用问题
            egui::ScrollArea::vertical().show(ui, |ui| {
                // 先收集需要执行的操作，避免在循环中修改
                let mut actions = Vec::new();
                
                let tasks = self.task_list.get_tasks().clone(); // 克隆任务列表
                
                if tasks.is_empty() {
                    ui.centered_and_justified(|ui| {
                        ui.label("🎉 暂无任务，添加一个开始吧！");
                    });
                } else {
                    for task in &tasks {
                        // 根据过滤条件决定是否显示
                        if !self.show_completed && task.is_completed() {
                            continue;
                        }

                        ui.group(|ui| {
                            ui.horizontal(|ui| {
                                // 任务状态指示器
                                let status_color = if task.is_completed() {
                                    egui::Color32::from_rgb(100, 200, 100) // 绿色
                                } else {
                                    egui::Color32::from_rgb(200, 200, 100) // 黄色
                                };
                                
                                let status_text = if task.is_completed() {
                                    "✅ 已完成"
                                } else {
                                    "⏳ 进行中"
                                };
                                
                                ui.colored_label(status_color, status_text);
                                
                                // 任务ID和描述
                                ui.label(format!("#{}", task.id()));
                                ui.label(&format!("{}", task));
                                
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    // 删除按钮
                                    if ui.button("🗑️ 删除").clicked() {
                                        actions.push(("delete", task.id()));
                                    }
                                    
                                    // 挂起按钮
                                    if !task.is_completed() && ui.button("⏸️ 挂起").clicked() {
                                        actions.push(("suspend", task.id()));
                                    }
                                    
                                    // 完成按钮
                                    if !task.is_completed() && ui.button("✅ 完成").clicked() {
                                        actions.push(("complete", task.id()));
                                    }
                                });
                            });
                        });
                        
                        ui.add_space(5.0);
                    }
                }
                
                // 在循环外执行收集到的操作
                for (action, task_id) in actions {
                    match action {
                        "delete" => self.delete_task(task_id),
                        "suspend" => self.suspend_task(task_id),
                        "complete" => self.complete_task(task_id),
                        _ => {}
                    }
                }
            });

            // 底部状态栏
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("状态:");
                ui.colored_label(egui::Color32::from_rgb(100, 150, 200), &self.status_message);
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("💾 手动保存").clicked() {
                        self.save_tasks();
                    }
                });
            });
        });
    }
}