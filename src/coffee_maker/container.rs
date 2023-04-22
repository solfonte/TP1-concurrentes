use std::sync::{Arc, Condvar, Mutex};

pub trait Container {
    fn extract(&self, extraction: u32) -> Result<u32, &str>;
    fn amount_left(&self) -> u32;

    /*fn get_resource_info(&self) -> (u32, u32) {
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

    }*/

    /*
    Condiciones
        -> busy && on -> espero
        -> !busy && on && !amount -> espero (por lo menos hasta que se apague)
        -> !on -> NO espero. Se apago y tendria que devolver false
        -> !busy && on && amount -> No espero. accedo

    */
}
