use std::sync::{Arc, Condvar, Mutex};

pub struct Container {
    pair: Arc<(Mutex<u32>, Condvar)>,
}

impl Container {
    pub fn new(max_capacity: u32) -> Self {
        Self {
            pair: Arc::new((Mutex::new(max_capacity), Condvar::new())),
        }
    }

    pub fn extract(&self, extract: u32) -> u32 {
        2
    }
}
