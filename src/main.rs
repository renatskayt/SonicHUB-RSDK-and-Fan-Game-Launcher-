mod config;
mod gamebanana;
mod ui;

use gtk4::prelude::*;
use gtk4::{Application, glib};

const APP_ID: &str = "dev.skayt.rsdk-launcher";

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(ui::build_ui);
    app.run()
}
