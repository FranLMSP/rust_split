use std::time::Instant;

use super::rs_split::RsSplit;

pub struct RsTimer {
    splits: Vec<RsSplit>,
    final_time: u128,
    record: u128,
    started: bool,
    finished: bool,
    time: Instant,
}

impl RsTimer {
    pub fn new() -> RsTimer {
        RsTimer {
            splits: Vec::new(),
            final_time: 0,
            record: 0,
            started: false,
            finished: false,
            time: Instant::now(),
        }
    }

    pub fn split(&mut self) {
        if self.is_finished() {
            ()
        }
        if !self.started {
            self.started = true;
            self.time = Instant::now();
        } else {
            self.finish();
        }
    }

    pub fn finish(&mut self) {
        let final_time = self.elapsed_ms();
        self.final_time = final_time;
        if final_time < self.record {
            self.record = final_time;
        }
        self.finished = true;
    }

    pub fn final_time(&self) -> u128 {
        self.final_time
    }

    pub fn record(&self) -> u128 {
        self.record
    }

    pub fn elapsed_ms(&self) -> u128 {
        self.time.elapsed().as_millis()
    }

    pub fn is_finished(&self) -> bool {
        self.finished
    }
}
