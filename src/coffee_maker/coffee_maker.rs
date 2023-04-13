use crate::{
    coffee_maker::container::Container, coffee_maker::dispenser::Dispenser, order::order::Order,
};
use std::{
    collections::VecDeque,
    sync::{Arc, Condvar, Mutex},
    thread::{self, JoinHandle},
};
use std_semaphore::Semaphore;
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
        Self {
            grain_container: Arc::new(Container::new(g)),
            ground_coffee_container: Arc::new(Container::new(m)),
            milk_container: Arc::new(Container::new(l)),
            milk_foam_container: Arc::new(Container::new(e)),
            cocoa_container: Arc::new(Container::new(c)),
            water_container: Arc::new(Container::new(a)),
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
            let ground_coffe_container_clone = self.ground_coffee_container.clone();
            let milk_foam_container_clone = self.milk_foam_container.clone();
            let water_container_clone = self.water_container.clone();
            let cocoa_container_clone = self.cocoa_container.clone();

            dispenser_vec.push(thread::spawn(move || {
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
                        order = order_queue.pop_front().unwrap();
                        println!(
                            "[dispenser number {i}] Order number {:?}",
                            order.get_order_number()
                        );
                    } else {
                        break; //TODO: manejar error
                    }
                    dispenser.prepare_order(order);
                }
            }))
        }
        /* TODO: join pero sino SE TRABA
        for handle in dispenser_vec {
            handle.join().unwrap();
        }*/
    }
}
