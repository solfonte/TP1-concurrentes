use std::sync::{Arc, Condvar, Mutex};

use super::container::Container;

#[derive(Debug)]
pub struct System {
    max_capacity: u32,
    amount: u32,
    busy: bool,
    is_on: bool,
}

pub struct NetworkRechargeableContainer {
    /* cantidad actual, is_on, is_busy */
    pair: Arc<(Mutex<System>, Condvar)>,
    name: String,
}

impl Container for NetworkRechargeableContainer {
    fn extract(&self, extraction: u32) -> Result<u32, &str> {
        let mut result = Ok(extraction);
        if let Ok(guard) = self.pair.0.lock() {
            if let Ok(mut system) = self.pair.1.wait_while(guard, |state| {
                (state.busy && state.is_on)
                    || (!state.busy && state.is_on && state.amount < extraction)
            }) {
                
                (*system).busy = true;
                
                if !(*system).is_on {
                    result = Ok(0);
                } else {
                    (*system).amount -= extraction;
                }
                
                if (*system).amount == 0 {
                    (*system).is_on = false;
                }
                
                println!("[container {}] {:?}", self.name, *system);
                (*system).busy = false;
            }
        }
        self.pair.1.notify_all();
        result
    }

    fn amount_left(&self) -> u32 {
        let mut amount_left = 0;
        if let Ok(guard) = self.pair.0.lock() {
            if let Ok(mut system) = self.pair.1.wait_while(guard, |state| {
                state.busy
            }) {
                (*system).busy = true;
                amount_left = (*system).amount;
                (*system).busy = false;
            }
        }
        amount_left
    }
}

impl NetworkRechargeableContainer {
    pub fn new(max_capacity: u32, name: String) -> Self {
        Self {
            pair: Arc::new((
                Mutex::new(System {
                    max_capacity,
                    amount: max_capacity,
                    busy: false,
                    is_on: true,
                }),
                Condvar::new(),
            )),
            name,
        }
    }
}
