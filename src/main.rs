use std::marker::PhantomData;

// States
struct Open;
struct Closed;
struct Locked;
struct Broken;

// Door struct
struct Door<State> {
    pin_code: String,
    _state: PhantomData<State>,
}

// Trait for actions that can be performed on the door
trait DoorAction {}

// Implement actions for each state
impl DoorAction for Open {}
impl DoorAction for Closed {}
impl DoorAction for Locked {}
impl DoorAction for Broken {}

// Door methods
impl Door<Closed> {
    fn new(pin_code: String) -> Self {
        Door {
            pin_code,
            _state: PhantomData,
        }
    }

    fn lock(self) -> Door<Locked> {
        println!("Locking the door.");
        Door {
            pin_code: self.pin_code,
            _state: PhantomData,
        }
    }

    fn open(self) -> Door<Open> {
        println!("Opening the door.");
        Door {
            pin_code: self.pin_code,
            _state: PhantomData,
        }
    }
}

impl Door<Locked> {
    fn unlock(self, entered_pin: &str) -> Result<Door<Closed>, &'static str> {
        if entered_pin == self.pin_code {
            println!("Unlocking the door.");
            Ok(Door {
                pin_code: self.pin_code,
                _state: PhantomData,
            })
        } else {
            Err("Incorrect PIN.")
        }
    }
}

impl Door<Open> {
    fn close(self) -> Door<Closed> {
        println!("Closing the door.");
        Door {
            pin_code: self.pin_code,
            _state: PhantomData,
        }
    }

    fn break_door(self) -> Door<Broken> {
        println!("The door is now broken.");
        Door {
            pin_code: self.pin_code,
            _state: PhantomData,
        }
    }
}

impl<State: DoorAction + 'static> Door<State> {
    fn state(&self) -> &'static str {
        match std::any::TypeId::of::<State>() {
            t if t == std::any::TypeId::of::<Open>() => "Open",
            t if t == std::any::TypeId::of::<Closed>() => "Closed",
            t if t == std::any::TypeId::of::<Locked>() => "Locked",
            t if t == std::any::TypeId::of::<Broken>() => "Broken",
            _ => "Unknown",
        }
    }
}

fn main() {
    let door = Door::new(String::from("1234"));
    println!("Current state: {}", door.state()); // "Current state: Closed"

    let door = door.lock();
    println!("Current state: {}", door.state()); // "Current state: Locked"

    let door = door.unlock("1234").unwrap();
    println!("Current state: {}", door.state()); // "Current state: Closed"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_door_states() {
        let door = Door::new(String::from("1234"));
        assert_eq!(door.state(), "Closed");

        let door = door.lock();
        assert_eq!(door.state(), "Locked");

        let door = door.unlock("1234").unwrap();
        assert_eq!(door.state(), "Closed");

        let door = door.open();
        assert_eq!(door.state(), "Open");

        let door = door.close();
        assert_eq!(door.state(), "Closed");
    }

    #[test]
    fn test_incorrect_pin() {
        let door = Door::new(String::from("1234")).lock();
        assert!(door.unlock("4321").is_err());
    }
}
