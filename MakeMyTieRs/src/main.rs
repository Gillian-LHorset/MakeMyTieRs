mod structurs;

fn main() -> eframe::Result {
    eframe::run_native(
        "MakeMyTieRs",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(MakeMyTieRs::default()))),
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
        
    }
}