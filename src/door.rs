use crate::error::DoorError;
use crate::states::{Broken, Closed, Locked, Open};

pub struct Door<State> {
    pin_code: String,
    pub state: State,
}

impl Door<Closed> {
    pub fn new(pin_code: String) -> Self {
        Door {
            pin_code,
            state: Closed::new(),
        }
    }

    pub fn lock(self) -> Door<Locked> {
        println!("Locking the door.");
        Door {
            pin_code: self.pin_code,
            state: Locked::new(),
        }
    }

    pub fn open(self) -> Door<Open> {
        println!("Opening the door.");
        Door {
            pin_code: self.pin_code,
            state: Open::new(),
        }
    }
}

impl Door<Locked> {
    pub fn unlock(self, entered_pin: &str) -> Result<Door<Closed>, DoorError> {
        if self.state.failed_attempts() >= 3 {
            return Err(DoorError::TooManyAttempts);
        }

        if entered_pin == self.pin_code {
            println!("Unlocking the door.");
            Ok(Door {
                pin_code: self.pin_code,
                state: Closed::from_locked(self.state),
            })
        } else {
            let mut state = self.state;
            state.increment_failed_attempts();
            Err(DoorError::IncorrectPin)
        }
    }
}

impl Door<Open> {
    pub fn close(self) -> Door<Closed> {
        println!("Closing the door.");
        Door {
            pin_code: self.pin_code,
            state: Closed::new(),
        }
    }

    pub fn break_door(self, reason: String) -> Door<Broken> {
        println!("The door is now broken. Reason: {}", reason);
        Door {
            pin_code: self.pin_code,
            state: Broken::new(reason),
        }
    }
}

impl<State: std::fmt::Debug> Door<State> {
    pub fn state_info(&self) -> String {
        format!("{:?}", self.state)
    }
}
