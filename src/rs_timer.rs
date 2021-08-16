use std::time::Instant;

use super::rs_split::RsSplit;

pub struct RsTimer {
    splits: Vec<RsSplit>,
    final_time: u128,
    record: u128,
    started: bool,
    finished: bool,
    split_pointer: usize,
    time: Instant,
}

impl RsTimer {
    pub fn new(splits: Vec<RsSplit>) -> Result<RsTimer, &'static str> {
        if splits.len() <= 0 {
            return Err("The timer needs to have at least one split!");
        }
        Ok(RsTimer {
            splits: splits,
            final_time: 0,
            record: 0,
            split_pointer: 0,
            started: false,
            finished: false,
            time: Instant::now(),
        })
    }

    pub fn split(&mut self) -> Result<(), &'static str> {
        if self.splits.len() <= 0 {
            return Err("You need to have at least one split!");
        }
        if self.is_finished() {
            return Ok(());
        }
        if !self.started {
            println!("Starting timer\r");
            self.started = true;
            self.time = Instant::now();
            self.split_pointer = 0;
            self.start_current_split()?;
            // If the timer just started, we don't need to do anything with the splits
            return Ok(());
        }
        if self.split_pointer <= self.last_pointer() {
            self.finish_current_split()?;

            if self.split_pointer < self.last_pointer() {
                println!("Starting new split\r");
                self.start_next_split()?;
            } else {
                self.finish();
            }
        }
        Ok(())
    }

    pub fn finish_current_split(&mut self) -> Result<(), &'static str> {
        let finish_time = self.elapsed_ms();
        if let Some(split) = self.splits.get_mut(self.split_pointer) {
            split.finish(finish_time);
        } else {
            return Err("Failed to get current split information!");
        }
        Ok(())
    }

    pub fn start_current_split(&mut self) -> Result<(), &'static str> {
        let start_time = self.elapsed_ms();
        if let Some(split) = self.splits.get_mut(self.split_pointer) {
            split.start(start_time);
        } else {
            return Err("Failed to get nextsplit information!");
        }
        Ok(())
    }

    pub fn start_next_split(&mut self) -> Result<(), &'static str> {
        let start_time = self.elapsed_ms();
        self.split_pointer += 1;
        if let Some(split) = self.splits.get_mut(self.split_pointer) {
            split.start(start_time);
        } else {
            return Err("Failed to get nextsplit information!");
        }
        Ok(())
    }

    pub fn finish(&mut self) {
        let final_time = self.elapsed_ms();
        self.final_time = final_time;
        if final_time < self.record {
            self.record = final_time;
        }
        self.finished = true;
        self.print_final_time();
    }

    pub fn final_time(&self) -> u128 {
        self.final_time
    }

    pub fn print_final_time(&self) {
        println!("FINAL TIME: {}ms\r", self.final_time());
    }

    pub fn record(&self) -> u128 {
        self.record
    }

    pub fn last_pointer(&self) -> usize {
        self.splits.len() - 1
    }

    pub fn elapsed_ms(&self) -> u128 {
        self.time.elapsed().as_millis()
    }

    pub fn is_finished(&self) -> bool {
        self.finished
    }
}
