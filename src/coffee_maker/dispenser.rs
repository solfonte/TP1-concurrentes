use crate::order::order::Order;
use std::sync::{Arc, Condvar, Mutex};

pub struct Dispenser {
    ground_coffee_container: Arc<(Mutex<u32>, Condvar)>,
    milk_foam_container: Arc<(Mutex<u32>, Condvar)>,
    water_container: Arc<(Mutex<u32>, Condvar)>,
    cocoa_container: Arc<(Mutex<u32>, Condvar)>,
}

impl Dispenser {
    pub fn new(
        ground_coffee_container: Arc<(Mutex<u32>, Condvar)>,
        milk_foam_container: Arc<(Mutex<u32>, Condvar)>,
        water_container: Arc<(Mutex<u32>, Condvar)>,
        cocoa_container: Arc<(Mutex<u32>, Condvar)>,
    ) -> Self {
        Self {
            ground_coffee_container,
            milk_foam_container,
            water_container,
            cocoa_container,
        }
    }

    pub fn prepare_order(&self, order: Order) {
        // empiezo a ver tema de recursos. TODO: ver si se hace concurrente
        if order.get_coffee_amount() > 0 {
            // agarro cafe
        }
        if order.get_cocoa_amount() > 0 {
            // agarro cacao
        }
        if order.get_milk_foam_amount() > 0 {
            // agarro espuma
        }
        if order.get_water_amount() > 0 {
            // agarro agua
        }
        //cuando se termina la imprimo.
    }
}
