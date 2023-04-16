use crate::{
    coffee_maker::{container::Container, dispenser::Dispenser},
    order::order::Order,
};
use std::{
    collections::VecDeque,
    sync::{Arc, Condvar, Mutex},
    thread::{self, JoinHandle},
};
use std_semaphore::Semaphore;

use super::{
    container_controller::{
        ContainerController, ContainerRechargerController, NormalContainerController,
        WaterNetworkController,
    },
    recharge_state::RechargeState,
};
//capaz no tienen que ser ARC los atributos, sino solo el coffee maker
pub struct CoffeeMaker {
    grain_container: Arc<Container>,
    ground_coffee_container: Arc<Container>,
    milk_container: Arc<Container>,
    milk_foam_container: Arc<Container>,
    cocoa_container: Arc<Container>,
    water_container: Arc<Container>,
    dispenser_amount: u32,
}

impl CoffeeMaker {
    pub fn new(g: u32, m: u32, l: u32, e: u32, c: u32, a: u32, n: u32) -> Self {
        let grain_container = Arc::new(Container::new(
            g,
            String::from("granos"),
            RechargeState::new(false, None),
        ));
        let milk_container = Arc::new(Container::new(
            l,
            String::from("milk"),
            RechargeState::new(false, None),
        ));
        let grain_container_clone = grain_container.clone();
        let milk_container_clone = milk_container.clone();

        Self {
            grain_container,
            ground_coffee_container: Arc::new(Container::new(
                m,
                String::from("cafe"),
                RechargeState::new(true, Some(grain_container_clone)),
            )),
            milk_container,
            milk_foam_container: Arc::new(Container::new(
                e,
                String::from("espuma"),
                RechargeState::new(true, Some(milk_container_clone)),
            )),
            cocoa_container: Arc::new(Container::new(
                c,
                String::from("cacao"),
                RechargeState::new(false, None),
            )),
            water_container: Arc::new(Container::new(
                a,
                String::from("agua"),
                RechargeState::new(false, None),
            )),
            dispenser_amount: n,
        }
    }

    fn turn_container_controllers_on(&self) -> Vec<JoinHandle<()>> {
        let mut controller_handles = Vec::new();

        let cocoa_container_clone = self.cocoa_container.clone();
        controller_handles.push(thread::spawn(move || {
            let cocoa_controller = NormalContainerController::new(cocoa_container_clone);
            cocoa_controller.turn_on();
        }));

        let coffee_container_clone = self.ground_coffee_container.clone();
        let grain_container_clone = self.grain_container.clone();
        controller_handles.push(thread::spawn(move || {
            let coffee_controller =
                ContainerRechargerController::new(coffee_container_clone, grain_container_clone);
            coffee_controller.turn_on();
        }));

        let milk_foam_container_clone = self.milk_foam_container.clone();
        let milk_container_clone = self.milk_container.clone();
        controller_handles.push(thread::spawn(move || {
            let foam_controller =
                ContainerRechargerController::new(milk_foam_container_clone, milk_container_clone);
            foam_controller.turn_on();
        }));

        let water_container_clone = self.water_container.clone();
        controller_handles.push(thread::spawn(move || {
            let water_controller = WaterNetworkController::new(water_container_clone);
            water_controller.turn_on();
        }));

        controller_handles
    }

    fn turn_dispensers_on(
        &self,
        order_queue_mutex: &Arc<Mutex<VecDeque<Order>>>,
        order_queue_semaphore: &Arc<Semaphore>,
    ) -> Vec<JoinHandle<u32>> {
        let mut dispenser_handles = Vec::new();
        for i in 0..self.dispenser_amount {
            let order_queue_mutex_clone = order_queue_mutex.clone();
            let order_queue_semaphore_clone = order_queue_semaphore.clone();
            let ground_coffe_container_clone = self.ground_coffee_container.clone();
            let milk_foam_container_clone = self.milk_foam_container.clone();
            let water_container_clone = self.water_container.clone();
            let cocoa_container_clone = self.cocoa_container.clone();

            dispenser_handles.push(thread::spawn(move || {
                println!("[dispenser {i}] turned on");
                let dispenser = Dispenser::new(
                    i,
                    ground_coffe_container_clone,
                    milk_foam_container_clone,
                    water_container_clone,
                    cocoa_container_clone,
                );
                loop {
                    order_queue_semaphore_clone.acquire();
                    let order;
                    if let Ok(mut order_queue) = order_queue_mutex_clone.lock() {
                        if let Some(_order) = order_queue.pop_front() {
                            //TODO: ver que otros resultaos tiene
                            order = _order;
                            println!(
                                "[dispenser number {i}] Order number {:?}",
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
                    if let Err(msg) = dispenser.prepare_order(order) {
                        println!("[OUT from dispenser {}] {} ", i, msg);
                    }
                }

                i
            }))
        }
        dispenser_handles
    }

    pub fn turn_on(
        &mut self,
        order_queue_mutex: Arc<Mutex<VecDeque<Order>>>,
        order_queue_semaphore: Arc<Semaphore>,
    ) {
        let controller_handles = self.turn_container_controllers_on();
        let dispenser_handles = self.turn_dispensers_on(&order_queue_mutex, &order_queue_semaphore);

        for c_handle in controller_handles {
            if let Ok(()) = c_handle.join() {
                println!("Turning controller off");
            }
        }
        for d_handle in dispenser_handles {
            if let Ok(dispenser_number) = d_handle.join() {
                println!("[dispenser {}] turned off", dispenser_number);
            }
        }
    }
}
