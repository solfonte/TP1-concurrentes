use crate::{coffee_maker::container::Container, order::order::Order};
use std::sync::{Arc, Condvar, Mutex};

pub struct Dispenser {
    dispenser_number: u32,
    ground_coffee_container: Arc<Container>,
    milk_foam_container: Arc<Container>,
    water_container: Arc<Container>,
    cocoa_container: Arc<Container>,
}

impl Dispenser {
    pub fn new(
        dispenser_number: u32,
        ground_coffee_container: Arc<Container>,
        milk_foam_container: Arc<Container>,
        water_container: Arc<Container>,
        cocoa_container: Arc<Container>,
    ) -> Self {
        Self {
            dispenser_number,
            ground_coffee_container,
            milk_foam_container,
            water_container,
            cocoa_container,
        }
    }

    pub fn prepare_order(&self, order: Order) {
        // empiezo a ver tema de recursos. TODO: ver si se hace concurrente
        let coffee_amount_required = order.get_coffee_amount();
        if coffee_amount_required > 0 {
            let coffee_taken = self.ground_coffee_container.extract(coffee_amount_required);
            // sleep
            println!(
                "[Dispenser {}] needed {} and took {} coffee",
                self.dispenser_number, coffee_amount_required, coffee_taken
            );
        }
        let cocoa_amount_required = order.get_cocoa_amount();
        if cocoa_amount_required > 0 {
            let cocoa_taken = self.cocoa_container.extract(cocoa_amount_required);
            // sleep
        }
        let milk_foam_amount_required = order.get_milk_foam_amount();
        if milk_foam_amount_required > 0 {
            let milk_foam_taken = self.milk_foam_container.extract(milk_foam_amount_required);
            // sleep
        }
        let water_amount_required = order.get_water_amount();
        if water_amount_required > 0 {
            let water_amount = self.water_container.extract(water_amount_required);
            // agarro agua
        }
        //cuando se termina la imprimo.
    }
}
