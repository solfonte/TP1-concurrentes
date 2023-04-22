use std_semaphore::Semaphore;

use crate::{coffee_maker::container::Container, order::{order::Order, order_system::OrderSystem, self}};
use std::{sync::{Arc, Condvar, Mutex}, collections::VecDeque};

use super::{
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

    pub fn take_order_from_queue(&self, order_queue_monitor: &Arc<(Mutex<OrderSystem>, Condvar)>) -> Option<Order> {

        let order ;
        let mut result = None;

        if let Ok(guard) = order_queue_monitor.0.lock() {
            if let Ok(mut order_system) = order_queue_monitor.1.wait_while(guard, |state| {
                state.is_busy() 
            }){
                if !order_system.there_are_orders_left() {
                    result = None;
                }else {
                    if let Some(_order) = order_system.get_order() {
                        order = _order;
                        println!(
                            "[dispenser number {}] Order number {:?}",
                            self.dispenser_number,
                            order.get_order_number()
                        );
                        result = Some(order);
                    }
                }
            }
        }
            order_queue_monitor.1.notify_all();
            result
    }

    
    pub fn process_order(&self, 
        coffee_container: &RechargeableContainer,
        foam_container: &RechargeableContainer,
        water_container: &NetworkRechargeableContainer,
        cocoa_container: &UnrechargeableContainer,
        order_queue_monitor: &Arc<(Mutex<OrderSystem>, Condvar)>) -> Result<bool, String> {
           // println!("[dispenser {}] enter to process", self.dispenser_number);

        if let Some (order) = self.take_order_from_queue(order_queue_monitor) {
           let result = self.prepare_order(
                order,
                &coffee_container,
                &foam_container,
                &water_container,
                &cocoa_container,
            );
            
            match result {
                Ok(_) => {
                    return Ok(false);
                }, 
                Err(msg) => {
                    return Err(String::from(msg))
                }
            } 
        }
        Ok(true)
    }
}

