use crate::{coffee_maker::{container::Container}, order::order::Order};
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

    pub fn dispense_ingredient(&self, ingredient_amount: u32, container: &Container) {
        if ingredient_amount > 0 {

            let ingredient_taken = container.extract(ingredient_amount);
            // sleep
            println!(
                "[Dispenser {}] needed {} and took {} ingredient",
                self.dispenser_number, ingredient_amount, ingredient_taken
            );
        }
    }

    pub fn prepare_order(&self, order: Order) {

        let coffee_amount_required = order.get_coffee_amount();
        self.dispense_ingredient(coffee_amount_required, &self.ground_coffee_container);
        
        let cocoa_amount_required = order.get_cocoa_amount();
        self.dispense_ingredient(cocoa_amount_required, &self.cocoa_container);
        
        let milk_foam_amount_required = order.get_milk_foam_amount();
        self.dispense_ingredient(milk_foam_amount_required, &self.milk_foam_container);
        
        let water_amount_required = order.get_water_amount();
        self.dispense_ingredient(water_amount_required, &self.water_container);
        
    }
}
