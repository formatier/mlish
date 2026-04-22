mod app;
mod generated;
pub mod ui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Mlish",
        native_options,
        Box::new(|cc| Ok(Box::new(app::MlishApp::new(&cc)))),
    )
    .unwrap();
}
