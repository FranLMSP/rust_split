use std::{thread, time};
use std::io::{stdout, Write};
use std::process;
use rust_split::rs_timer::RsTimer;
use rust_split::rs_split::RsSplit;

use crossterm::{
    event::{poll, read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};



fn main() -> Result<()> {
    let splits = vec![
        RsSplit::new("First split", None, None),
        RsSplit::new("Second split", None, None),
        RsSplit::new("Third split", None, None),
    ];
    let mut rs_timer = RsTimer::new(splits).unwrap_or_else(|e| {
        eprintln!("Application error: {}", e);
        process::exit(1);
    });

    enable_raw_mode()?;

    loop {
        // Wait up to 1s for another event
        if poll(time::Duration::from_millis(1_000))? {
            // It's guaranteed that read() wont block if `poll` returns `Ok(true)`
            let event = read()?;

            // println!("Event::{:?}\r", event);

            if event == Event::Key(KeyCode::Char(' ').into()) && !rs_timer.is_finished() {
                match rs_timer.split() {
                    Err(e) => eprintln!("Application error: {}", e),
                    Ok(_) => (),
                };
            }

            if event == Event::Key(KeyCode::Backspace.into()) {
                match rs_timer.undo() {
                    Err(e) => eprintln!("Application error: {}", e),
                    Ok(_) => (),
                };
            }

            if event == Event::Key(KeyCode::Tab.into()) {
                match rs_timer.skip() {
                    Err(e) => eprintln!("Application error: {}", e),
                    Ok(_) => (),
                };
            }

            if event == Event::Key(KeyCode::Esc.into()) {
                break;
            }
        } else {
            // Timeout expired, no event for 1s
            // println!(".\r");
        }
    }

    disable_raw_mode()?;

    Ok(())
}

