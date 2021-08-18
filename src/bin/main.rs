use std::process;
use rust_split::rs_timer::RsTimer;
use rust_split::rs_split::RsSplit;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

fn main() {
    let splits = vec![
        RsSplit::new("First split", None, None),
        RsSplit::new("Second split", None, None),
        RsSplit::new("Third split", None, None),
    ];
    let mut rs_timer = RsTimer::new(splits).unwrap_or_else(|e| {
        eprintln!("Application error: {}", e);
        process::exit(1);
    });

    let app = Application::builder()
        .application_id("org.gtk.rust_split")
        .build();
    
    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("RustSplit")
        .build();

    window.present();
}
