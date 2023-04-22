use crate::{
    coffee_maker::{
        dispenser::Dispenser,
        network_rechargeable_container::NetworkRechargeableContainer,
        rechargeable_container::RechargeableContainer,
        unrechargeable_container::UnrechargeableContainer,
    },
    order::order_system::OrderSystem,
};
use std::{
    sync::{Arc, Mutex, Condvar},
    thread::{self, JoinHandle},
};

const COFFEE_RECHARGING_RATE: u32 = 10;
const FOAM_RECHARGING_RATE: u32 = 5;

use super::{provider_container::ProviderContainer, container_rechargeable_controller::ContainerRechargerController};
//capaz no tienen que ser ARC los atributos, sino solo el coffee maker

fn start_dispenser(
    dispenser_number: u32,
    order_queue_monitor: Arc<(Mutex<OrderSystem>, Condvar)>,
    ground_coffee_container: Arc<RechargeableContainer>,
    milk_foam_container: Arc<RechargeableContainer>,
    cocoa_container: Arc<UnrechargeableContainer>,
    water_container: Arc<NetworkRechargeableContainer>,
) -> JoinHandle<u32> {
    thread::spawn(move || {
        //println!("[dispenser {dispenser_number}] turned on");
        let dispenser = Dispenser::new(dispenser_number);
        let ground_coffee_container_clone = ground_coffee_container.clone();
        let milk_foam_container_clone = milk_foam_container.clone();
        let water_container_clone = water_container.clone();
        let cocoa_container = cocoa_container.clone();

        let mut finish_processing_orders: bool = false;
        while !finish_processing_orders {
            if let Ok(finished_processing) = dispenser.process_order(&ground_coffee_container_clone,
                &milk_foam_container_clone,
                &water_container_clone,
                &cocoa_container,
                &order_queue_monitor){
                    finish_processing_orders = finished_processing;
            }else {
                    /*condicion de error */
                finish_processing_orders = true
            }            
        }

        dispenser_number
    })
}

pub struct CoffeeMaker {
    max_grain_capacity: u32,
    max_milk_capacity: u32,
    max_grounded_coffe_capacity: u32,
    max_milk_foam_capacity: u32,
    max_cocoa_capacity: u32,
    max_water_capacity: u32,
    dispenser_amount: u32,
}

impl CoffeeMaker {
    pub fn new(g: u32, m: u32, l: u32, e: u32, c: u32, a: u32, n: u32) -> Self {
        Self {
            max_grain_capacity: g,
            max_milk_capacity: l,
            max_grounded_coffe_capacity: m,
            max_milk_foam_capacity: e,
            max_cocoa_capacity: c,
            max_water_capacity: a,
            dispenser_amount: n,
        }
    }

    fn turn_dispensers_on(
        &self,
        ground_coffee_container: Arc<RechargeableContainer>,
        milk_foam_container: Arc<RechargeableContainer>,
        cocoa_container: Arc<UnrechargeableContainer>,
        water_container: Arc<NetworkRechargeableContainer>,
        order_queue_monitor: &Arc<(Mutex<OrderSystem>, Condvar)>,
    ) -> Vec<JoinHandle<u32>> {
        let mut dispenser_handles = Vec::new();
        for i in 0..self.dispenser_amount {
            let dispenser_handle = start_dispenser(
                i,
                order_queue_monitor.clone(),
                ground_coffee_container.clone(),
                milk_foam_container.clone(),
                cocoa_container.clone(),
                water_container.clone(),
            );
            dispenser_handles.push(dispenser_handle)
        }
        dispenser_handles
    }

    pub fn turn_on(
        &mut self,
        order_queue_monitor: Arc<(Mutex<OrderSystem>, Condvar)>,
    ) {
        let grain_container = Arc::new(ProviderContainer::new(
            self.max_grain_capacity,
            String::from("granos"),
        ));
        let milk_container = Arc::new(ProviderContainer::new(
            self.max_milk_capacity,
            String::from("milk"),
        ));
        let ground_coffee_container = Arc::new(RechargeableContainer::new(
            self.max_grounded_coffe_capacity,
            String::from("cafe"),
            ContainerRechargerController::new(grain_container.clone(), String::from("Coffee rechager")),
            COFFEE_RECHARGING_RATE
        ));
        let milk_foam_container = Arc::new(RechargeableContainer::new(
            self.max_milk_foam_capacity,
            String::from("espuma"),
            ContainerRechargerController::new(milk_container.clone(), String::from("Foam rechager")),
            FOAM_RECHARGING_RATE
        ));
        let cocoa_container = Arc::new(UnrechargeableContainer::new(
            self.max_cocoa_capacity,
            String::from("cacao"),
        ));
        let water_container = Arc::new(NetworkRechargeableContainer::new(
            self.max_water_capacity,
            String::from("agua"),
        ));

        let dispenser_handles = self.turn_dispensers_on(
            ground_coffee_container,
            milk_foam_container,
            cocoa_container,
            water_container,
            &order_queue_monitor
        );

        for d_handle in dispenser_handles {
            if let Ok(dispenser_number) = d_handle.join() {
                //println!("[dispenser {}] turned off", dispenser_number);
            }
        }
    }
}
