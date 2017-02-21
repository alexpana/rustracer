extern crate time;

pub struct Timer {
    start_ns: u64
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            start_ns: time::precise_time_ns()
        }
    }

    pub fn start(&mut self) {
        self.start_ns = time::precise_time_ns();
    }

    pub fn count(&self) -> u64 {
        return time::precise_time_ns() - self.start_ns;
    }

    pub fn count_seconds(&self) -> f64 {
        return (self.count() as f64) / 1000_000_000.0;
    }
}