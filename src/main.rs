mod door;
mod error;
mod states;

use door::Door;
use error::DoorError;
use states::{Broken, Closed, Locked, Open};

fn main() -> Result<(), DoorError> {
    let door = Door::new(String::from("1234"));
    println!("Initial state: {}", door.state_info());

    let door: Door<Locked> = door.lock();
    println!("After locking: {}", door.state_info());

    let door: Door<Closed> = door.unlock("1234")?;
    println!("After unlocking: {}", door.state_info());

    let door: Door<Open> = door.open();
    println!("After opening: {}", door.state_info());

    let door: Door<Closed> = door.close();
    println!("After closing: {}", door.state_info());

    let door: Door<Broken> = door.open().break_door("Handle fell off".to_string());
    println!("After breaking: {}", door.state_info());

    // Attempt to lock an already locked door (to demonstrate error handling)
    let door: Door<Locked> = Door::new(String::from("5678")).lock();
    match door.unlock("wrong_pin") {
        Ok(_) => println!("This shouldn't happen"),
        Err(DoorError::IncorrectPin) => println!("Incorrect PIN"),
        Err(e) => println!("Unexpected error: {:?}", e),
    }

    Ok(())
}
