use std::time::Instant;

use super::rs_split::RsSplit;

pub struct RsTimer {
    splits: Vec<RsSplit>,
    final_time: u128,
    record: u128,
    new_record: u128,
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
            new_record: 0,
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
        if self.split_pointer <= self.last_split_pointer() {
            self.finish_current_split()?;

            if self.split_pointer < self.last_split_pointer() {
                println!("Starting new split\r");
                self.start_next_split()?;
            } else {
                self.finish();
            }
        }
        Ok(())
    }

    pub fn undo(&mut self) -> Result<(), &'static str> {
        if self.split_pointer <= 0 {
            return Ok(());
        }
        let current_split = self.get_current_split()?;
        let split_finished = current_split.is_finished();
        if self.split_pointer == self.last_split_pointer() && split_finished {
            self.finished = false;
            self.final_time = 0;
            self.new_record = 0;
            self.undo_current_split()?;
            self.print_undo_message()?;
            return Ok(());
        }
        self.reset_current_split()?;
        self.split_pointer -= 1;
        self.undo_current_split()?;
        self.print_undo_message()?;
        Ok(())
    }

    pub fn skip(&mut self) -> Result<(), &'static str> {
        if self.split_pointer == self.last_split_pointer() {
            return Ok(());
        }
        let finish_time = self.elapsed_ms();
        let split = self.get_current_split()?;
        split.skip(finish_time);
        self.start_next_split()?;
        Ok(())
    }

    fn print_undo_message(&mut self) -> Result<(), &'static str> {
        let split = self.get_current_split()?;
        println!("Went back to split: {}\r", split.name());
        Ok(())
    }

    fn reset_current_split(&mut self) -> Result<(), &'static str> {
        let split = self.get_current_split()?;
        split.reset();
        Ok(())
    }

    fn undo_current_split(&mut self) -> Result<(), &'static str> {
        let split = self.get_current_split()?;
        split.undo();
        Ok(())
    }

    fn finish_current_split(&mut self) -> Result<(), &'static str> {
        let finish_time = self.elapsed_ms();
        let split = self.get_current_split()?;
        split.finish(finish_time);
        Ok(())
    }

    fn start_current_split(&mut self) -> Result<(), &'static str> {
        let start_time = self.elapsed_ms();
        let split = self.get_current_split()?;
        split.start(start_time);
        Ok(())
    }

    fn start_next_split(&mut self) -> Result<(), &'static str> {
        let start_time = self.elapsed_ms();
        self.split_pointer += 1;
        let split = self.get_current_split()?;
        split.start(start_time);
        Ok(())
    }

    pub fn finish(&mut self) {
        let final_time = self.elapsed_ms();
        self.final_time = final_time;
        if final_time < self.record {
            self.new_record = final_time;
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

    fn last_split_pointer(&self) -> usize {
        self.splits.len() - 1
    }

    pub fn elapsed_ms(&self) -> u128 {
        self.time.elapsed().as_millis()
    }

    pub fn is_finished(&self) -> bool {
        self.finished
    }

    fn get_current_split(&mut self) -> Result<&mut RsSplit, &'static str> {
        self.get_split(self.split_pointer)
    }

    fn get_split(&mut self, ptr: usize) -> Result<&mut RsSplit, &'static str> {
        match self.splits.get_mut(ptr) {
            Some(split) => Ok(split),
            None => Err("Failed to get split information!"),
        }
    }
}
