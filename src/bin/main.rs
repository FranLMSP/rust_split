use std::process;
use rust_split::rs_timer::RsTimer;
use rust_split::rs_split::RsSplit;

use druid::{AppLauncher, WindowDesc};

use rust_split::data::AppState;
use rust_split::data::TodoItem;
use rust_split::view::build_ui;
use rust_split::delegate::Delegate;

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

    let main_window = WindowDesc::new(build_ui)
        .title("RustSplit")
        .window_size((400.0, 400.0));

    let todos = vec![TodoItem::new("Thing one"), TodoItem::new("Thing two")];
    let initial_state = AppState::load_from_json();

    AppLauncher::with_window(main_window)
        .delegate(Delegate {})
        .launch(initial_state)
        .expect("Failed to launch application");
}
