use crate::{coffee_maker::container::Container, order::order::Order};
use std::sync::{Arc, Condvar, Mutex};

use super::{
    container_rechargeable_controller::ContainerRechargerController,
    network_rechargeable_container::NetworkRechargeableContainer,
    rechargeable_container::RechargeableContainer,
    unrechargeable_container::UnrechargeableContainer,
};

pub struct Dispenser {
    dispenser_number: u32,
}

impl Dispenser {
    pub fn new(dispenser_number: u32) -> Self {
        Self { dispenser_number }
    }

    pub fn dispense_resource<T: Container>(
        &self,
        ingredient_amount: u32,
        container: &T,
    ) -> Result<u32, &str> {
        let extraction_result = container.extract(ingredient_amount);
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

    pub fn prepare_order(
        &self,
        order: Order,
        coffee_container: &RechargeableContainer,
        foam_container: &RechargeableContainer,
        water_container: &NetworkRechargeableContainer,
        cocoa_container: &UnrechargeableContainer,
        grain_controller: &ContainerRechargerController,
        milk_controller: &ContainerRechargerController,
    ) -> Result<u32, String> {
        //TODO: aca deberia validar lo que devuelve
        //  -> devuelve que un contenedor se apago -> no se puede seguir esa orden AHI VIENE LA PARTE DEL CRITERIO
        //  -> devuelve que me dio bien el ingrediente
        let coffee_amount_required = order.get_coffee_amount();
        if coffee_amount_required > 0 {
            if let Err(msg) = self.dispense_resource(coffee_amount_required, coffee_container) {
                return Err(String::from(msg));
            }
        }

        let cocoa_amount_required = order.get_cocoa_amount();
        if cocoa_amount_required > 0 {
            if let Err(msg) = self.dispense_resource(cocoa_amount_required, cocoa_container) {
                return Err(String::from(msg));
            }
        }

        let milk_foam_amount_required = order.get_milk_foam_amount();
        if milk_foam_amount_required > 0 {
            if let Err(msg) = self.dispense_resource(milk_foam_amount_required, foam_container) {
                return Err(String::from(msg));
            }
        }

        let water_amount_required = order.get_water_amount();
        if water_amount_required > 0 {
            if let Err(msg) = self.dispense_resource(water_amount_required, water_container) {
                return Err(String::from(msg));
            }
        }

        //TODO:check por Ok(0)
        println!("[FINISHED] ORDER {}", order.get_order_number());
        Ok(1)
    }
}
