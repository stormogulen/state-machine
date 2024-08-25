#[derive(Debug)]
#[allow(dead_code)]
pub enum DoorError {
    AlreadyLocked,
    AlreadyUnlocked,
    IncorrectPin,
    TooManyAttempts,
    BrokenCannotOperate,
}
