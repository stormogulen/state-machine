#[derive(Debug)]
pub struct Broken {
    reason: String,
}

impl Broken {
    pub fn new(reason: String) -> Self {
        Broken { reason }
    }
}
