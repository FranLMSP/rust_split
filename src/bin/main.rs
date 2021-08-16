use std::{thread, time};
use std::io::{stdout, Write};
use rust_split::rs_timer::RsTimer;

use crossterm::{
    event::{poll, read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};



fn main() -> Result<()> {
    let mut rs_timer = RsTimer::new();

    enable_raw_mode()?;

    loop {
        // Wait up to 1s for another event
        if poll(time::Duration::from_millis(1_000))? {
            // It's guaranteed that read() wont block if `poll` returns `Ok(true)`
            let event = read()?;

            // println!("Event::{:?}\r", event);

            if event == Event::Key(KeyCode::Char(' ').into()) && !rs_timer.is_finished() {
                println!("Started!");
                rs_timer.split();
                if rs_timer.is_finished() {
                    println!("time in ms: {}", rs_timer.final_time());
                }
            }
            println!("\r");

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

