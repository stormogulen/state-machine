use super::Locked;
use std::time::Instant;

#[derive(Debug)]
pub struct Closed {
    #[allow(dead_code)]
    last_closed: Instant,
}

impl Closed {
    pub fn new() -> Self {
        Closed {
            last_closed: Instant::now(),
        }
    }

    pub fn from_locked(locked: Locked) -> Self {
        Closed {
            last_closed: locked.lock_time(),
        }
    }
}
