use iced::window;

mod app;
mod generated;
mod plugin;
mod stylesheets;

fn main() {
    iced::application(move || app::App::new(), app::App::update, app::App::view)
        .theme(app::App::theme)
        .title(app::App::title)
        .run()
        .unwrap();
}
