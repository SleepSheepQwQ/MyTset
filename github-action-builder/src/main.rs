//! Desktop test version of GitHub Action Builder
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 600.0])
            .with_title("GitHub Action Builder - Desktop Test"),
        ..Default::default()
    };
    
    eframe::run_native(
        "GitHub Action Builder",
        options,
        Box::new(|cc| Box::new(GitHubActionApp::new(cc))),
    )
}

struct GitHubActionApp {
    projects: Vec<String>,
}

impl GitHubActionApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            projects: vec!["my-repo".to_string(), "another-project".to_string()],
        }
    }
}

impl eframe::App for GitHubActionApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🚀 GitHub Action Builder");
            ui.label("Manage your GitHub Actions from mobile");
            
            ui.separator();
            
            ui.heading("Your Projects:");
            for project in &self.projects {
                ui.horizontal(|ui| {
                    ui.label(project);
                    if ui.button("Trigger Build").clicked() {
                        println!("Would trigger build for: {}", project);
                    }
                });
            }
            
            if ui.button("Add Project").clicked() {
                self.projects.push("new-project".to_string());
            }
        });
    }
}