mod structurs;

fn main() -> eframe::Result {
    eframe::run_native(
        "MakeMyTieRs",
        eframe::NativeOptions::default(),
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx.clone());
            Ok(Box::new(MakeMyTieRs::default()))
        }),
    )
}

enum Screen {
    Home,
    Editor,
}

impl Default for Screen {
    fn default() -> Self {
        Screen::Home
    }
}

#[derive(Default)]
struct MakeMyTieRs {}

impl eframe::App for MakeMyTieRs {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.add(
                egui::Image::from_bytes( 
                    "logo.png",
                    include_bytes!("../../Ressources/MakeMyTieRs-Logo.png"),
                )
                .max_width(200.0),
            );
        });
    }
}
