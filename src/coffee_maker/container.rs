use std::{
    fmt::Error,
    sync::{Arc, Condvar, Mutex},
};

#[derive(Debug)]
pub struct System {
    max_capacity: u32,
    amount: u32,
    busy: bool,
    is_on: bool,
}
pub struct Container {
    /* cantidad actual, is_on, is_busy */
    pair: Arc<(Mutex<System>, Condvar)>,
    name: String,
}

impl Container {
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

    pub fn get_resource_info(&self) -> (u32, u32) {
        let mut max_capacity = 0;
        let mut available_resource = 0;
        if let Ok(guard) = self.pair.0.lock() {
            if let Ok(mut system) = self.pair.1.wait_while(guard, |state| state.busy) {
                (*system).busy = true;
                max_capacity = (*system).max_capacity;
                available_resource = (*system).amount;
                (*system).busy = false;
            }
        }
        self.pair.1.notify_all();
        (max_capacity, available_resource)
    }

    pub fn extract(&self, extraction: u32) -> Result<u32, &str> {
        let mut result = Ok(extraction);
        if let Ok(guard) = self.pair.0.lock() {
            if let Ok(mut system) = self.pair.1.wait_while(guard, |state| {
                (state.busy && state.is_on)
                    || (!state.busy && state.is_on && state.amount < extraction)
            }) {
                println!("[container {}] {:?}", self.name, *system);

                (*system).busy = true;

                if !(*system).is_on {
                    result = Ok(0);
                } else {
                    (*system).amount -= extraction;
                }

                if (*system).amount == 0 {
                    (*system).is_on = false;
                }

                (*system).busy = false;
            }
        }
        self.pair.1.notify_all();
        result
    }
}

/*
Condiciones
    -> busy && on -> espero
    -> !busy && on && !amount -> espero (por lo menos hasta que se apague)
    -> !on -> NO espero. Se apago y tendria que devolver false
    -> !busy && on && amount -> No espero. accedo

*/
