use std::{
    sync::{Arc, Condvar, Mutex},
    thread,
    time::Duration,
};

use super::container::Container;
const NETWORK_LOADING_RATE: u64 = 2; // 2 units per second

#[derive(Debug)]
pub struct System {
    max_capacity: u32,
    amount: u32,
    busy: bool,
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
            if let Ok(mut system) = self.pair.1.wait_while(guard, |state| state.busy) {
                (*system).busy = true;
                if (*system).max_capacity >= extraction && (*system).amount < extraction {
                    (*system).amount = self.recharge_from_network(
                        (*system).max_capacity,
                        (*system).max_capacity - (*system).amount,
                    );
                }

                if (*system).amount >= extraction {
                    (*system).amount -= extraction;
                    result = Ok(extraction);
                } else {
                    result = Ok(0);
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
            if let Ok(mut system) = self.pair.1.wait_while(guard, |state| state.busy) {
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
                }),
                Condvar::new(),
            )),
            name,
        }
    }

    fn recharge_from_network(&self, max_capacity: u32, amount_to_recharge: u32) -> u32 {
        thread::sleep(Duration::from_secs(
            (amount_to_recharge as u64) / NETWORK_LOADING_RATE,
        ));
        max_capacity
    }
}
