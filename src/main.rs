use eframe::egui;
use std::path::{Path, PathBuf};
use std::collections::HashSet;

mod task;
mod file_import;
mod file_export;
mod file_export_xml;
mod mspdi;
mod merge;

use task::Task;
use file_import::{import_xlsx, import_mpp};
use file_export::export_to_xlsx;
use file_export_xml::export_to_xml;
use egui_extras::{TableBuilder, Column};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("MS Project Merger"),
        ..Default::default()
    };

    eframe::run_native(
        "MS Project Merger",
        options,
        Box::new(|cc| Ok(Box::new(MsProjectMergerApp::new(cc)))),
    )
}

#[derive(PartialEq)]
enum WizardStep {
    SelectFiles,
    Review,
    Export,
}

struct MsProjectMergerApp {
    tasks: Vec<Task>,
    file_paths: Vec<PathBuf>,
    selected_tasks: HashSet<usize>, // Changed type to HashSet
    sort_column: Option<usize>,
    sort_ascending: bool,
    filter_text: String,
    show_import_dialog: bool,
    show_export_dialog: bool,
    export_path: String,
    current_step: WizardStep,
    export_success_msg: Option<String>,
    selected_task_for_edit: Option<usize>,
    edit_panel_open: bool,
}

impl MsProjectMergerApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize visuals (Professional Dark Mode)
        let mut visuals = egui::Visuals::dark();
        visuals.window_rounding = egui::Rounding::same(12.0);
        visuals.menu_rounding = egui::Rounding::same(8.0);
        
        // Deep charcoal background
        visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(26, 27, 38); // #1A1B26
        visuals.widgets.noninteractive.fg_stroke.color = egui::Color32::from_rgb(192, 202, 245); // #C0CAF5
        
        // Surface color for panels/windows
        visuals.panel_fill = egui::Color32::from_rgb(26, 27, 38);
        
        // Selection & Accents
        visuals.selection.bg_fill = egui::Color32::from_rgb(65, 72, 104); // #414868
        visuals.selection.stroke.color = egui::Color32::from_rgb(122, 162, 247); // #7AA2F7
        
        // Button styling
        visuals.widgets.inactive.weak_bg_fill = egui::Color32::from_rgb(36, 40, 59); // #24283B
        visuals.widgets.inactive.rounding = egui::Rounding::same(6.0);
        visuals.widgets.hovered.weak_bg_fill = egui::Color32::from_rgb(65, 72, 104);
        visuals.widgets.hovered.rounding = egui::Rounding::same(6.0);
        visuals.widgets.active.weak_bg_fill = egui::Color32::from_rgb(86, 95, 137);
        visuals.widgets.active.rounding = egui::Rounding::same(6.0);

        cc.egui_ctx.set_visuals(visuals);

        // Set a default font size that is slightly larger for readability
        let mut style = (*cc.egui_ctx.style()).clone();
        style.text_styles.insert(
            egui::TextStyle::Body,
            egui::FontId::new(14.0, egui::FontFamily::Proportional),
        );
        style.text_styles.insert(
            egui::TextStyle::Heading,
            egui::FontId::new(24.0, egui::FontFamily::Proportional),
        );
        style.spacing.item_spacing = egui::vec2(8.0, 8.0);
        cc.egui_ctx.set_style(style);

        Self {
            tasks: Vec::new(),
            file_paths: Vec::new(),
            filter_text: String::new(),
            show_import_dialog: false,
            show_export_dialog: false,
            export_path: String::new(),
            selected_tasks: HashSet::new(), // Initialized as HashSet
            sort_column: None,
            sort_ascending: true,
            current_step: WizardStep::SelectFiles,
            export_success_msg: None,
            selected_task_for_edit: None,
            edit_panel_open: false,
        }
    }
}

impl eframe::App for MsProjectMergerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::none().inner_margin(8.0).fill(ctx.style().visuals.window_fill()))
            .show(ctx, |ui| {
                match self.current_step {
                    WizardStep::SelectFiles => {
                        ui.vertical_centered(|ui| {
                            ui.add_space(20.0);
                            ui.heading(egui::RichText::new("Step 1: Select Project Files").size(24.0));
                            ui.add_space(10.0);
                            ui.label("Add the files you want to merge. The first file will be the Base Project.");
                            ui.add_space(20.0);

                            if ui.button(egui::RichText::new("➕ Add Project File").size(18.0)).clicked() {
                                self.show_import_dialog = true;
                            }
                        });

                        ui.add_space(20.0);
                        ui.separator();
                        ui.add_space(10.0);

                        // List of files
                        if !self.file_paths.is_empty() {
                            ui.label(egui::RichText::new("Selected Files:").strong());
                            let mut to_remove = None;
                            
                            for (i, path) in self.file_paths.iter().enumerate() {
                                ui.horizontal(|ui| {
                                    let label = if i == 0 {
                                        egui::RichText::new("Base Project").color(egui::Color32::GREEN).strong()
                                    } else {
                                        egui::RichText::new("Overlay").color(egui::Color32::LIGHT_BLUE)
                                    };
                                    ui.label(label);
                                    ui.label(path.file_name().unwrap_or_default().to_string_lossy());
                                    
                                    if ui.small_button("❌").clicked() {
                                        to_remove = Some(i);
                                    }
                                });
                            }

                            if let Some(idx) = to_remove {
                                self.file_paths.remove(idx);
                                // Remove tasks from this file
                                self.tasks.retain(|t| t.source_file != idx);
                                // Update source_file indices
                                for task in &mut self.tasks {
                                    if task.source_file > idx {
                                        task.source_file -= 1;
                                    }
                                }
                            }
                        } else {
                            ui.centered_and_justified(|ui| {
                                ui.label(egui::RichText::new("No files selected yet.").italics().color(egui::Color32::GRAY));
                            });
                        }

                        ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                            ui.add_space(20.0);
                            if ui.add_enabled(
                                self.file_paths.len() >= 2,
                                egui::Button::new(egui::RichText::new("Next: Merge Files ➡").size(18.0))
                            ).clicked() {
                                self.merge_loaded_files();
                                self.current_step = WizardStep::Review;
                            }
                        });
                    }
                    WizardStep::Review => {
                        ui.horizontal(|ui| {
                            if ui.button("⬅ Back").clicked() {
                                self.current_step = WizardStep::SelectFiles;
                                self.edit_panel_open = false;
                                self.selected_task_for_edit = None;
                            }
                            ui.heading("Step 2: Review Merged Data");
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.button(egui::RichText::new("Next: Export ➡").strong()).clicked() {
                                    self.current_step = WizardStep::Export;
                                }
                                ui.add(egui::TextEdit::singleline(&mut self.filter_text).hint_text("🔍 Filter tasks..."));
                            });
                        });
                        ui.separator();
                        
                        // Split layout: Table on left, Edit panel on right (if open)
                        if self.edit_panel_open && self.selected_task_for_edit.is_some() {
                            egui::SidePanel::right("edit_panel")
                                .resizable(true)
                                .default_width(400.0)
                                .show_inside(ui, |ui| {
                                    self.show_edit_panel(ui);
                                });
                        }
                        
                        egui::CentralPanel::default().show_inside(ui, |ui| {
                            self.show_task_table(ui);
                        });
                    }
                    WizardStep::Export => {
                        ui.vertical_centered(|ui| {
                            ui.add_space(40.0);
                            ui.heading(egui::RichText::new("Step 3: Export").size(24.0));
                            ui.add_space(20.0);
                            ui.label("Your merged project is ready to be exported.");
                            ui.add_space(30.0);

                            if ui.button(egui::RichText::new("💾 Export to MSPDI (XML)").size(20.0)).clicked() {
                                self.export_path = "merged_project.xml".to_string(); // Default name
                                self.show_export_dialog = true;
                            }
                            
                            ui.add_space(10.0);
                            
                            if ui.button("Export to Excel").clicked() {
                                self.export_path = "merged_project.xlsx".to_string();
                                self.show_export_dialog = true;
                            }

                            if let Some(msg) = &self.export_success_msg {
                                ui.add_space(20.0);
                                ui.label(egui::RichText::new(msg).color(egui::Color32::GREEN).strong());
                            }

                            ui.add_space(40.0);
                            if ui.button("🔄 Start Over").clicked() {
                                self.tasks.clear();
                                self.file_paths.clear();
                                self.selected_tasks.clear();
                                self.current_step = WizardStep::SelectFiles;
                                self.export_success_msg = None;
                            }
                            
                            ui.add_space(10.0);
                            if ui.button("⬅ Back to Review").clicked() {
                                self.current_step = WizardStep::Review;
                            }
                        });
                    }
                }
            });

        // Dialogs
        if self.show_import_dialog {
            self.show_import_dialog(ctx);
        }
        if self.show_export_dialog {
            self.show_export_dialog(ctx);
        }
    }
}

impl MsProjectMergerApp {
    fn merge_loaded_files(&mut self) {
        // Strategy:
        // 1. Separate tasks by source file
        // 2. Take file 0 as base
        // 3. Merge file 1, then file 2, etc. into base
        
        if self.file_paths.is_empty() {
            return;
        }

        let mut base_tasks: Vec<Task> = self.tasks.iter()
            .filter(|t| t.source_file == 0)
            .cloned()
            .collect();

        for i in 1..self.file_paths.len() {
            let overlay_tasks: Vec<Task> = self.tasks.iter()
                .filter(|t| t.source_file == i)
                .cloned()
                .collect();
            
            crate::merge::merge_projects(&mut base_tasks, &overlay_tasks);
        }
        
        // Replace all tasks with the merged result
        // Note: We lose the source_file distinction for the merged result, 
        // essentially everything becomes "Base" (source 0) or we could mark them differently.
        // For now, let's just update the main list.
        self.tasks = base_tasks;
        
        // Optional: Clear other files from the list since they are merged?
        // Or keep them? Let's keep them but maybe update the UI to show they are merged.
        // Actually, simpler to just replace the view with the merged result.
    }
    fn show_task_table(&mut self, ui: &mut egui::Ui) {
        let mut filtered_tasks: Vec<(usize, Task)> = self
            .tasks
            .iter()
            .enumerate()
            .filter(|(_, task)| {
                if self.filter_text.is_empty() {
                    true
                } else {
                    let filter = self.filter_text.to_lowercase();
                    task.name.to_lowercase().contains(&filter)
                        || task.description.to_lowercase().contains(&filter)
                        || task.assignee.to_lowercase().contains(&filter)
                }
            })
            .map(|(i, t)| (i, t.clone()))
            .collect();

        // Apply sorting
        if let Some(col) = self.sort_column {
            filtered_tasks.sort_by(|a, b| {
                let cmp = match col {
                    0 => a.1.name.cmp(&b.1.name),
                    1 => a.1.start_date.cmp(&b.1.start_date),
                    2 => a.1.end_date.cmp(&b.1.end_date),
                    3 => a.1.status.cmp(&b.1.status),
                    4 => a.1.priority.cmp(&b.1.priority),
                    _ => std::cmp::Ordering::Equal,
                };
                if self.sort_ascending {
                    cmp
                } else {
                    cmp.reverse()
                }
            });
        }

        let text_height = egui::TextStyle::Body.resolve(ui.style()).size + 8.0; // Increased row height

        TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto().resizable(false)) // Selection
            .column(Column::initial(300.0).resizable(true)) // Name
            .column(Column::initial(100.0).resizable(true)) // Start
            .column(Column::initial(100.0).resizable(true)) // End
            .column(Column::initial(120.0).resizable(true)) // Status (Wider for badge)
            .column(Column::initial(80.0).resizable(true)) // Priority
            .column(Column::initial(150.0).resizable(true)) // Assignee
            .column(Column::auto().resizable(false)) // Edit button
            .header(30.0, |mut header| { // Taller header
                header.col(|ui| { ui.label(""); });
                header.col(|ui| { 
                    if ui.button(egui::RichText::new("Name").strong()).clicked() { 
                        self.sort_ascending = if self.sort_column == Some(0) { !self.sort_ascending } else { true };
                        self.sort_column = Some(0);
                    }
                });
                header.col(|ui| { 
                    if ui.button(egui::RichText::new("Start").strong()).clicked() { 
                        self.sort_ascending = if self.sort_column == Some(1) { !self.sort_ascending } else { true };
                        self.sort_column = Some(1);
                    }
                });
                header.col(|ui| { 
                    if ui.button(egui::RichText::new("End").strong()).clicked() { 
                        self.sort_ascending = if self.sort_column == Some(2) { !self.sort_ascending } else { true };
                        self.sort_column = Some(2);
                    }
                });
                header.col(|ui| { 
                    if ui.button(egui::RichText::new("Status").strong()).clicked() { 
                        self.sort_ascending = if self.sort_column == Some(3) { !self.sort_ascending } else { true };
                        self.sort_column = Some(3);
                    }
                });
                header.col(|ui| { 
                    if ui.button(egui::RichText::new("Priority").strong()).clicked() { 
                        self.sort_ascending = if self.sort_column == Some(4) { !self.sort_ascending } else { true };
                        self.sort_column = Some(4);
                    }
                });
                header.col(|ui| { ui.label(egui::RichText::new("Assignee").strong()); });
                header.col(|ui| { ui.label(""); });
            })
            .body(|mut body| {
                for (original_idx, task) in filtered_tasks {
                    body.row(text_height, |mut row| {
                        row.col(|ui| {
                            let mut selected = self.selected_tasks.contains(&original_idx);
                            if ui.checkbox(&mut selected, "").changed() {
                                if selected {
                                    self.selected_tasks.insert(original_idx);
                                } else {
                                    self.selected_tasks.remove(&original_idx);
                                }
                            }
                        });
                        row.col(|ui| { ui.label(&task.name); });
                        row.col(|ui| { ui.label(task.start_date.format("%Y-%m-%d").to_string()); });
                        row.col(|ui| { ui.label(task.end_date.format("%Y-%m-%d").to_string()); });
                        row.col(|ui| { 
                            // Status Badge
                            let (bg_color, text_color, text) = match task.status {
                                crate::task::TaskStatus::Completed => (egui::Color32::from_rgb(20, 80, 40), egui::Color32::from_rgb(150, 255, 150), "Completed"),
                                crate::task::TaskStatus::InProgress => (egui::Color32::from_rgb(20, 60, 100), egui::Color32::from_rgb(150, 200, 255), "In Progress"),
                                crate::task::TaskStatus::NotStarted => (egui::Color32::from_rgb(60, 60, 60), egui::Color32::from_rgb(200, 200, 200), "Not Started"),
                                _ => (egui::Color32::GRAY, egui::Color32::WHITE, "Unknown"),
                            };
                            
                            egui::Frame::none()
                                .fill(bg_color)
                                .rounding(12.0)
                                .inner_margin(egui::vec2(8.0, 2.0))
                                .show(ui, |ui| {
                                    ui.label(egui::RichText::new(text).color(text_color).size(12.0));
                                });
                        });
                        row.col(|ui| { ui.label(task.priority.to_string()); });
                        row.col(|ui| { ui.label(&task.assignee); });
                        row.col(|ui| {
                            if ui.small_button("✏ Edit").clicked() {
                                self.selected_task_for_edit = Some(original_idx);
                                self.edit_panel_open = true;
                            }
                        });
                    });
                }
            });

        ui.separator();

        // Action buttons for selected tasks
        if !self.selected_tasks.is_empty() {
            ui.horizontal(|ui| {
                ui.label(format!("{} task(s) selected", self.selected_tasks.len()));
                if ui.button("Delete Selected").clicked() {
                    let mut indices_to_remove: Vec<usize> = self.selected_tasks.iter().copied().collect();
                    indices_to_remove.sort_unstable_by(|a, b| b.cmp(a)); // Sort descending
                    for idx in indices_to_remove {
                        self.tasks.remove(idx);
                    }
                    self.selected_tasks.clear();
                }
                if ui.button("Move Up").clicked() {
                    self.move_selected_tasks_up();
                }
                if ui.button("Move Down").clicked() {
                    self.move_selected_tasks_down();
                }
            });
        }
    }

    fn move_selected_tasks_up(&mut self) {
        let mut indices: Vec<usize> = self.selected_tasks.iter().copied().collect();
        indices.sort();
        
        for &idx in &indices {
            if idx > 0 && !indices.contains(&(idx - 1)) {
                self.tasks.swap(idx, idx - 1);
                // Update selection
                self.selected_tasks.remove(&idx);
                self.selected_tasks.insert(idx - 1);
            }
        }
    }

    fn move_selected_tasks_down(&mut self) {
        let mut indices: Vec<usize> = self.selected_tasks.iter().copied().collect();
        indices.sort_by(|a, b| b.cmp(a)); // Sort descending
        
        for &idx in &indices {
            if idx < self.tasks.len() - 1 && !indices.contains(&(idx + 1)) {
                self.tasks.swap(idx, idx + 1);
                // Update selection
                self.selected_tasks.remove(&idx);
                self.selected_tasks.insert(idx + 1);
            }
        }
    }

    fn show_edit_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("Edit Task");
        ui.separator();

        if let Some(idx) = self.selected_task_for_edit {
            if idx < self.tasks.len() {
                // Clone the task data to avoid borrow checker issues
                let mut task_name = self.tasks[idx].name.clone();
                let mut task_desc = self.tasks[idx].description.clone();
                let mut task_assignee = self.tasks[idx].assignee.clone();
                let mut task_start = self.tasks[idx].start_date;
                let mut task_end = self.tasks[idx].end_date;
                let mut task_status = self.tasks[idx].status.clone();
                let mut task_priority = self.tasks[idx].priority;
                let task_predecessors = self.tasks[idx].predecessors.clone();
                
                let mut should_delete = false;
                let mut should_close = false;
                let mut has_changes = false;

                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add_space(10.0);

                    // Basic Fields
                    ui.label(egui::RichText::new("Basic Information").strong());
                    ui.add_space(5.0);

                    ui.label("Name:");
                    if ui.text_edit_singleline(&mut task_name).changed() {
                        has_changes = true;
                    }
                    ui.add_space(5.0);

                    ui.label("Description:");
                    if ui.text_edit_multiline(&mut task_desc).changed() {
                        has_changes = true;
                    }
                    ui.add_space(5.0);

                    ui.label("Assignee:");
                    if ui.text_edit_singleline(&mut task_assignee).changed() {
                        has_changes = true;
                    }
                    ui.add_space(10.0);

                    // Dates
                    ui.label(egui::RichText::new("Dates").strong());
                    ui.add_space(5.0);

                    ui.label("Start Date (YYYY-MM-DD):");
                    let mut start_str = task_start.format("%Y-%m-%d").to_string();
                    if ui.text_edit_singleline(&mut start_str).changed() {
                        if let Ok(date) = chrono::NaiveDate::parse_from_str(&start_str, "%Y-%m-%d") {
                            task_start = date;
                            has_changes = true;
                        }
                    }
                    ui.add_space(5.0);

                    ui.label("End Date (YYYY-MM-DD):");
                    let mut end_str = task_end.format("%Y-%m-%d").to_string();
                    if ui.text_edit_singleline(&mut end_str).changed() {
                        if let Ok(date) = chrono::NaiveDate::parse_from_str(&end_str, "%Y-%m-%d") {
                            task_end = date;
                            has_changes = true;
                        }
                    }
                    ui.add_space(10.0);

                    // Status and Priority
                    ui.label(egui::RichText::new("Status & Priority").strong());
                    ui.add_space(5.0);

                    ui.label("Status:");
                    if egui::ComboBox::from_id_salt("status_combo")
                        .selected_text(format!("{:?}", task_status))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut task_status, crate::task::TaskStatus::NotStarted, "NotStarted");
                            ui.selectable_value(&mut task_status, crate::task::TaskStatus::InProgress, "InProgress");
                            ui.selectable_value(&mut task_status, crate::task::TaskStatus::Completed, "Completed");
                            ui.selectable_value(&mut task_status, crate::task::TaskStatus::OnHold, "OnHold");
                        })
                        .response.changed() {
                        has_changes = true;
                    }
                    ui.add_space(5.0);

                    ui.label("Priority:");
                    if ui.add(egui::Slider::new(&mut task_priority, 1..=10)).changed() {
                        has_changes = true;
                    }
                    ui.add_space(10.0);

                    // Dependencies
                    ui.label(egui::RichText::new("Dependencies").strong());
                    ui.add_space(5.0);

                    if task_predecessors.is_empty() {
                        ui.label(egui::RichText::new("No dependencies").italics().color(egui::Color32::GRAY));
                    } else {
                        for pred in &task_predecessors {
                            ui.horizontal(|ui| {
                                ui.label(format!("Predecessor UID: {}", pred.predecessor_uid));
                                ui.label(format!("Type: {}", pred.link_type));
                            });
                        }
                    }

                    ui.add_space(10.0);

                    // Action Buttons
                    ui.separator();
                    ui.add_space(10.0);

                    ui.horizontal(|ui| {
                        if ui.button(egui::RichText::new("✓ Close").strong()).clicked() {
                            should_close = true;
                        }
                        
                        if ui.button("Delete Task").clicked() {
                            should_delete = true;
                        }
                    });
                });

                // Apply changes after the borrow ends
                if has_changes && idx < self.tasks.len() {
                    self.tasks[idx].name = task_name;
                    self.tasks[idx].description = task_desc;
                    self.tasks[idx].assignee = task_assignee;
                    self.tasks[idx].start_date = task_start;
                    self.tasks[idx].end_date = task_end;
                    self.tasks[idx].status = task_status;
                    self.tasks[idx].priority = task_priority;
                }

                if should_delete {
                    self.tasks.remove(idx);
                    self.edit_panel_open = false;
                    self.selected_task_for_edit = None;
                } else if should_close {
                    self.edit_panel_open = false;
                    self.selected_task_for_edit = None;
                }
            }
        }
    }

    fn show_import_dialog(&mut self, ctx: &egui::Context) {
        egui::Window::new("Import Files")
            .collapsible(false)
            .resizable(true)
            .show(ctx, |ui| {
                ui.label("Select MPP or XLSX files to import:");

                ui.horizontal(|ui| {
                    if ui.button("Select Files").clicked() {
                        if let Some(paths) = rfd::FileDialog::new()
                            .add_filter("Project Files", &["mpp", "xlsx", "xml"])
                            .pick_files()
                        {
                            for path in paths {
                                if !self.file_paths.contains(&path) {
                                    self.file_paths.push(path.clone());
                                    let file_idx = self.file_paths.len() - 1;
                                    
                                    // Import tasks based on file extension
                                    if path.extension().and_then(|s| s.to_str()) == Some("xlsx") {
                                        if let Ok(mut tasks) = import_xlsx(&path) {
                                            for task in &mut tasks {
                                                task.source_file = file_idx;
                                            }
                                            self.tasks.append(&mut tasks);
                                        } else {
                                            eprintln!("Failed to import XLSX file: {}", path.display());
                                        }
                                    } else if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                                        if ext == "mpp" || ext == "xml" {
                                            if let Ok(mut tasks) = import_mpp(&path) {
                                                for task in &mut tasks {
                                                    task.source_file = file_idx;
                                                }
                                                self.tasks.append(&mut tasks);
                                            } else {
                                                eprintln!("Failed to import MPP/XML file: {}", path.display());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                });

                ui.separator();

                if ui.button("Close").clicked() {
                    self.show_import_dialog = false;
                }
            });
    }

    fn show_export_dialog(&mut self, ctx: &egui::Context) {
        egui::Window::new("Export Tasks")
            .collapsible(false)
            .resizable(true)
            .show(ctx, |ui| {
                ui.label("Export path:");
                ui.text_edit_singleline(&mut self.export_path);

                ui.horizontal(|ui| {
                    if ui.button("Browse").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("Project Files", &["xml", "xlsx"])
                            .save_file()
                        {
                            self.export_path = path.to_string_lossy().to_string();
                        }
                    }
                });

                ui.separator();

                ui.horizontal(|ui| {
                    if ui.button("Export").clicked() {
                        if !self.export_path.is_empty() {
                            let result = if self.export_path.ends_with(".xlsx") {
                                export_to_xlsx(&self.tasks, &self.export_path)
                            } else {
                                // Default to XML if no extension or .xml
                                let path = if !self.export_path.contains('.') {
                                    format!("{}.xml", self.export_path)
                                } else {
                                    self.export_path.clone()
                                };
                                export_to_xml(&self.tasks, &path)
                            };

                            if let Err(e) = result {
                                eprintln!("Export error: {}", e);
                            } else {
                                self.show_export_dialog = false;
                                self.export_path.clear();
                            }
                        }
                    }
                    if ui.button("Cancel").clicked() {
                        self.show_export_dialog = false;
                        self.export_path.clear();
                    }
                });
            });
    }
}

