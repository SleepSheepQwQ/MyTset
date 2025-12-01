use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([420.0, 680.0])
            .with_title("Actions监控 - 桌面测试"),
        ..Default::default()
    };

    eframe::run_native(
        "Actions监控",
        options,
        Box::new(|_cc| Box::new(App::默认())),
    )
}

struct App {
    关键字: String,
    日志示例: Vec<String>,
}

impl App {
    fn 默认() -> Self {
        Self {
            关键字: String::new(),
            日志示例: vec![
                "INFO: 构建开始".to_string(),
                "WARN: 依赖下载缓慢".to_string(),
                "ERROR: 单元测试失败".to_string(),
            ],
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("GitHub Actions 日志过滤（示例）");
            ui.horizontal(|ui| {
                ui.label("关键字：");
                ui.text_edit_singleline(&mut self.关键字);
            });
            ui.separator();
            for 行 in self.日志示例.iter().filter(|s| {
                if self.关键字.is_empty() { true } else { s.contains(&self.关键字) }
            }) {
                ui.label(行);
            }
        });
    }
}