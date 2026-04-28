mod app;
mod generated;
mod stylesheets;

fn main() {
    iced::application(move || app::App::new(), app::App::update, app::App::view)
        .run()
        .unwrap();
}
