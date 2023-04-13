use std::sync::{Arc, Condvar, Mutex};

#[derive(Debug)]
pub struct System {
    amount: u32,
    busy: bool,
    is_on: bool
}
pub struct Container { /* cantidad actual, is_on, is_busy */
    pair: Arc<(Mutex<System>, Condvar)>,
}

impl Container {
    pub fn new(max_capacity: u32) -> Self {
        Self {
            pair: Arc::new((Mutex::new(System {amount: max_capacity, busy: false, is_on: true}), Condvar::new())),
        }
    }

    pub fn extract(&self, extraction: u32) -> u32 {

        let mut possible_extraction = 0;
        if let Ok(guard) = self.pair.0.lock() {
            if let Ok(mut system) =  self.pair.1.wait_while(guard,|state| {
                    println!("[Contenedor] Chequeando {:?}", state);
                    state.busy
            }){
                (*system).busy = true;

                if (*system).amount >= extraction {
                    (*system).amount -= extraction;
                    possible_extraction = extraction;
                }else {
                    (*system).is_on = false;
                }
                (*system).busy = false;
                self.pair.1.notify_all();
            }
        }
        possible_extraction 
    }
    
}
