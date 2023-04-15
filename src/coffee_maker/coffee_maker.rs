use crate::{
    coffee_maker::{
        container::Container, container_controller::ContainerController, dispenser::Dispenser,
    },
    order::order::Order,
};
use std::{
    collections::VecDeque,
    sync::{Arc, Condvar, Mutex},
    thread::{self, JoinHandle},
};
use std_semaphore::Semaphore;

//capaz no tienen que ser ARC los atributos, sino solo el coffee maker
pub struct CoffeeMaker {
    coffee_controller: Arc<ContainerController>,
    foam_controller: Arc<ContainerController>,
    cocoa_controller: Arc<ContainerController>,
    water_controller: Arc<ContainerController>,
    dispenser_amount: u32,
}

impl CoffeeMaker {
    pub fn new(g: u32, m: u32, l: u32, e: u32, c: u32, a: u32, n: u32) -> Self {
        Self {
            coffee_controller: Arc::new(ContainerController::new(g, m, String::from("Coffe"))),
            foam_controller: Arc::new(ContainerController::new(l, e, String::from("Foam"))),
            cocoa_controller: Arc::new(ContainerController::new(c, 0, String::from("Cocoa"))),
            water_controller: Arc::new(ContainerController::new(a, 0, String::from("Water"))),
            dispenser_amount: n,
        }
    }

    pub fn turn_on(
        &mut self,
        order_queue_mutex: Arc<Mutex<VecDeque<Order>>>,
        order_queue_semaphore: Arc<Semaphore>,
    ) {
        let mut dispenser_vec = Vec::new();

        for i in 0..self.dispenser_amount {
            let order_queue_mutex_clone = order_queue_mutex.clone();
            let order_queue_semaphore_clone = order_queue_semaphore.clone();
            let coffee_controller_clone = self.coffee_controller.clone();
            let foam_controller_clone = self.foam_controller.clone();
            let water_controller_clone = self.water_controller.clone();
            let cocoa_controller_clone = self.cocoa_controller.clone();

            dispenser_vec.push(thread::spawn(move || {
                println!("[dispenser {i}] turned on");
                let dispenser = Dispenser::new(
                    i,
                    coffee_controller_clone,
                    foam_controller_clone,
                    water_controller_clone,
                    cocoa_controller_clone,
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

        for handle in dispenser_vec {
            if let Ok(dispenser_number) = handle.join() {
                println!("[dispenser {}] turned off", dispenser_number);
            }
        }
    }
}
