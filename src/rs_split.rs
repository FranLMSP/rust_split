pub struct RsSplit {
    name: String,
    start_time: u128,
    finish_time: u128,
    gold: Option<u128>,
    new_gold: Option<u128>,
    pb: Option<u128>,
    new_pb: Option<u128>,
    skipped: bool,
    started: bool,
    finished: bool,
}

impl RsSplit {
    pub fn new(name: &str, gold: Option<u128>, pb: Option<u128>) -> RsSplit {
        RsSplit {
            name: String::from(name),
            start_time: 0,
            finish_time: 0,
            gold: gold,
            new_gold: None,
            pb: pb,
            new_pb: None,
            skipped: false,
            started: false,
            finished: false,
        }
    }

    pub fn print_time(&self) {
        if self.skipped {
            println!("{}: -\r", self.name);
            return;
        }
        println!("{}: {}ms\r", self.name, self.time());
    }

    pub fn start(&mut self, start_time: u128) {
        self.started = true;
        self.start_time = start_time;
    }

    pub fn skip(&mut self, finish_time: u128) {
        self.finish_time = finish_time;
        self.new_pb = None;
        self.new_gold = None;
        self.skipped = true;
        self.print_time();
    }

    pub fn reset(&mut self) {
        self.start_time = 0;
        self.finish_time = 0;
        self.new_gold = None;
        self.new_pb = None;
        self.started = false;
        self.skipped = false;
        self.finished = false;
    }

    pub fn undo(&mut self) {
        self.finish_time = 0;
        self.new_pb = None;
        self.new_gold = None;
        self.skipped = false;
        self.finished = false;
    }

    pub fn finish(&mut self, finish_time: u128) {
        self.finish_time = finish_time;
        let time = self.time();
        match self.gold {
            Some(gold) => {
                if gold > time {
                    self.new_gold = Some(time);
                }
            },
            None => {
                self.new_gold = Some(time);
            }
        }
        match self.pb {
            Some(pb) => {
                if pb > time {
                    self.new_pb = Some(time);
                }
            },
            None => {
                self.new_gold = Some(time);
            }
        }
        self.finished = true;
        self.print_time();
    }

    pub fn time(&self) -> u128 {
        if self.finish_time < self.start_time {
            return 0;
        }
        self.finish_time - self.start_time
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_finished(&self) -> bool {
        self.finished
    }
}
