use std::time::Instant;

#[derive(Debug)]
pub struct Locked {
    lock_time: Instant,
    failed_attempts: u32,
}

impl Locked {
    pub fn new() -> Self {
        Locked {
            lock_time: Instant::now(),
            failed_attempts: 0,
        }
    }

    pub fn failed_attempts(&self) -> u32 {
        self.failed_attempts
    }

    pub fn increment_failed_attempts(&mut self) {
        self.failed_attempts += 1;
    }

    pub fn lock_time(&self) -> Instant {
        self.lock_time
    }
}
