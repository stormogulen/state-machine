use std::time::Instant;

#[derive(Debug)]
pub struct Open {
    time_opened: Instant,
}

impl Open {
    pub fn new() -> Self {
        Open {
            time_opened: Instant::now(),
        }
    }
}
