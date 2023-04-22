use std::sync::{Arc, Condvar, Mutex};

use super::container::Container;

#[derive(Debug)]
pub struct System {
    amount: u32,
    busy: bool,
    is_on: bool,
}

pub struct UnrechargeableContainer {
    /* cantidad actual, is_on, is_busy */
    pair: Arc<(Mutex<System>, Condvar)>,
    name: String,
}

impl Container for UnrechargeableContainer {
    fn extract(&self, extraction: u32) -> Result<u32, &str> {
        let mut result: Result<u32, &str> = Err("No se pudo extraer del contenedor");
        if let Ok(guard) = self.pair.0.lock() {
            if let Ok(mut system) = self.pair.1.wait_while(guard, |state| {
                state.busy && state.is_on
            }) {
                
                (*system).busy = true;
                
                if (*system).amount < extraction {
                    result = Ok(0);
                } else {
                    (*system).amount -= extraction;
                    result = Ok(extraction);
                }
                
                (*system).busy = false;
                println!("[container {}] {:?}", self.name, *system);
            }
        }
        self.pair.1.notify_all();
        result
    }
}

impl UnrechargeableContainer {
    pub fn new(max_capacity: u32, name: String) -> Self {
        Self {
            pair: Arc::new((
                Mutex::new(System {
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
