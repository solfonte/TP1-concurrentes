use crate::{
    coffee_maker::{
        container::Container, dispenser::Dispenser,
        network_rechargeable_container::NetworkRechargeableContainer,
        rechargeable_container::RechargeableContainer,
        unrechargeable_container::UnrechargeableContainer,
    },
    order::order::Order,
};
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex, WaitTimeoutResult},
    thread::{self, JoinHandle},
};
use std_semaphore::Semaphore;

use super::container_rechargeable_controller::ContainerRechargerController;
//capaz no tienen que ser ARC los atributos, sino solo el coffee maker

fn start_dispenser(
    dispenser_number: u32,
    order_queue_mutex: Arc<Mutex<VecDeque<Order>>>,
    order_queue_semaphore: Arc<Semaphore>,
    ground_coffee_container: Arc<RechargeableContainer>,
    milk_foam_container: Arc<RechargeableContainer>,
    cocoa_container: Arc<UnrechargeableContainer>,
    water_container: Arc<NetworkRechargeableContainer>,
    grain_controller: Arc<ContainerRechargerController>,
    milk_controller: Arc<ContainerRechargerController>,
) -> JoinHandle<u32> {
    thread::spawn(move || {
        println!("[dispenser {dispenser_number}] turned on");
        let dispenser = Dispenser::new(dispenser_number);
        let grain_controller_clone = grain_controller.clone();
        let milk_controller_clone = milk_controller.clone();
        let ground_coffee_container_clone = ground_coffee_container.clone();
        let milk_foam_container_clone = milk_foam_container.clone();
        let water_container_clone = water_container.clone();
        let cocoa_container = cocoa_container.clone();
        loop {
            order_queue_semaphore.acquire();
            let order;
            if let Ok(mut order_queue) = order_queue_mutex.lock() {
                if let Some(_order) = order_queue.pop_front() {
                    //TODO: ver que otros resultaos tiene
                    order = _order;
                    println!(
                        "[dispenser number {dispenser_number}] Order number {:?}",
                        order.get_order_number()
                    );
                } else {
                    println!("Se ejecuto el break porque hubo error");
                    break;
                }
            } else {
                println!("Se ejecuto el break porque hubo error");
                break;
            }
            if let Err(msg) = dispenser.prepare_order(
                order,
                &ground_coffee_container_clone,
                &milk_foam_container_clone,
                &water_container_clone,
                &cocoa_container,
                &grain_controller_clone,
                &milk_controller_clone,
            ) {
                println!("[OUT from dispenser {}] {} ", dispenser_number, msg);
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
        grain_container: Arc<UnrechargeableContainer>,
        milk_container: Arc<UnrechargeableContainer>,
        ground_coffee_container: Arc<RechargeableContainer>,
        milk_foam_container: Arc<RechargeableContainer>,
        cocoa_container: Arc<UnrechargeableContainer>,
        water_container: Arc<NetworkRechargeableContainer>,
        order_queue_mutex: &Arc<Mutex<VecDeque<Order>>>,
        order_queue_semaphore: &Arc<Semaphore>,
    ) -> Vec<JoinHandle<u32>> {
        let mut dispenser_handles = Vec::new();
        let grain_controller = Arc::new(ContainerRechargerController::new(grain_container.clone()));

        let milk_controller = Arc::new(ContainerRechargerController::new(milk_container.clone()));
        for i in 0..self.dispenser_amount {
            let dispenser_handle = start_dispenser(
                i,
                order_queue_mutex.clone(),
                order_queue_semaphore.clone(),
                ground_coffee_container.clone(),
                milk_foam_container.clone(),
                cocoa_container.clone(),
                water_container.clone(),
                grain_controller.clone(),
                milk_controller.clone(),
            );
            dispenser_handles.push(dispenser_handle)
        }
        dispenser_handles
    }

    pub fn turn_on(
        &mut self,
        order_queue_mutex: Arc<Mutex<VecDeque<Order>>>,
        order_queue_semaphore: Arc<Semaphore>,
    ) {
        let grain_container = Arc::new(UnrechargeableContainer::new(
            self.max_grain_capacity,
            String::from("granos"),
        ));
        let milk_container = Arc::new(UnrechargeableContainer::new(
            self.max_milk_capacity,
            String::from("milk"),
        ));
        let ground_coffee_container = Arc::new(RechargeableContainer::new(
            self.max_grounded_coffe_capacity,
            String::from("cafe"),
            ContainerRechargerController::new(grain_container.clone()),
        ));
        let milk_foam_container = Arc::new(RechargeableContainer::new(
            self.max_milk_foam_capacity,
            String::from("espuma"),
            ContainerRechargerController::new(milk_container.clone()),
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
            grain_container,
            milk_container,
            ground_coffee_container,
            milk_foam_container,
            cocoa_container,
            water_container,
            &order_queue_mutex,
            &order_queue_semaphore,
        );

        for d_handle in dispenser_handles {
            if let Ok(dispenser_number) = d_handle.join() {
                println!("[dispenser {}] turned off", dispenser_number);
            }
        }
    }
}
