use crate::{
    coffee_maker::{container::Container, container_controller::ContainerController},
    order::order::Order,
};
use std::sync::{Arc, Condvar, Mutex};

pub struct Dispenser {
    dispenser_number: u32,
    coffee_container: Arc<Container>,
    foam_container: Arc<Container>,
    water_container: Arc<Container>,
    cocoa_container: Arc<Container>,
}

impl Dispenser {
    pub fn new(
        dispenser_number: u32,
        coffee_container: Arc<Container>,
        foam_container: Arc<Container>,
        water_container: Arc<Container>,
        cocoa_container: Arc<Container>,
    ) -> Self {
        Self {
            dispenser_number,
            coffee_container,
            foam_container,
            water_container,
            cocoa_container,
        }
    }

    pub fn dispense_ingredient(
        &self,
        ingredient_amount: u32,
        controller: &Container,
    ) -> Result<u32, &str> {
        let extraction_result = controller.extract(ingredient_amount);
        match extraction_result {
            Ok(ingredient_taken) => {
                // sleep
                return Ok(ingredient_taken);
            }
            Err(msg) => {
                println!("[Error extracting]{msg}");
                return Err("");
            }
        }
    }

    pub fn prepare_order(&self, order: Order) -> Result<u32, String> {
        //TODO: aca deberia validar lo que devuelve
        //  -> devuelve que un contenedor se apago -> no se puede seguir esa orden AHI VIENE LA PARTE DEL CRITERIO
        //  -> devuelve que me dio bien el ingrediente
        let coffee_amount_required = order.get_coffee_amount();
        if coffee_amount_required > 0 {
            if let Err(msg) =
                self.dispense_ingredient(coffee_amount_required, &self.coffee_container)
            {
                return Err(String::from(msg));
            }
        }

        let cocoa_amount_required = order.get_cocoa_amount();
        if cocoa_amount_required > 0 {
            if let Err(msg) = self.dispense_ingredient(cocoa_amount_required, &self.cocoa_container)
            {
                return Err(String::from(msg));
            }
        }

        let milk_foam_amount_required = order.get_milk_foam_amount();
        if milk_foam_amount_required > 0 {
            if let Err(msg) =
                self.dispense_ingredient(milk_foam_amount_required, &self.foam_container)
            {
                return Err(String::from(msg));
            }
        }

        let water_amount_required = order.get_water_amount();
        if water_amount_required > 0 {
            if let Err(msg) = self.dispense_ingredient(water_amount_required, &self.water_container)
            {
                return Err(String::from(msg));
            }
        }

        //TODO:check por Ok(0)
        println!("[FINISHED] ORDER {}", order.get_order_number());
        Ok(1)
    }
}
