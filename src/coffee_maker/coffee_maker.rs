use crate::{coffee_maker::dispenser::Dispenser, order::order::Order};
use std::{
    collections::VecDeque,
    sync::{Arc, Condvar, Mutex},
    thread::{self, JoinHandle},
};
use std_semaphore::Semaphore;
//capaz no tienen que ser ARC los atributos, sino solo el coffee maker
pub struct CoffeeMaker {
    grain_container: Arc<(Mutex<u32>, Condvar)>,
    ground_coffee_container: Arc<(Mutex<u32>, Condvar)>,
    milk_container: Arc<(Mutex<u32>, Condvar)>,
    milk_foam_container: Arc<(Mutex<u32>, Condvar)>,
    cocoa_container: Arc<(Mutex<u32>, Condvar)>,
    water_container: Arc<(Mutex<u32>, Condvar)>,
    dispenser_amount: u32,
    dispenser_vec: Vec<JoinHandle<()>>,
}

impl CoffeeMaker {
    pub fn new(g: u32, m: u32, l: u32, e: u32, c: u32, a: u32, n: u32) -> Self {
        Self {
            grain_container: Arc::new((Mutex::new(g), Condvar::new())),
            ground_coffee_container: Arc::new((Mutex::new(m), Condvar::new())),
            milk_container: Arc::new((Mutex::new(l), Condvar::new())),
            milk_foam_container: Arc::new((Mutex::new(e), Condvar::new())),
            cocoa_container: Arc::new((Mutex::new(c), Condvar::new())),
            water_container: Arc::new((Mutex::new(a), Condvar::new())),
            dispenser_amount: n,
            dispenser_vec: Vec::new(),
        }
    }

    pub fn turn_on(
        &mut self,
        order_queue_mutex: Arc<Mutex<VecDeque<Order>>>,
        order_queue_semaphore: Arc<Semaphore>,
    ) {
        for i in 0..self.dispenser_amount {
            let order_queue_mutex_clone = order_queue_mutex.clone();
            let order_queue_semaphore_clone = order_queue_semaphore.clone();
            let ground_coffe_container_clone = self.ground_coffee_container.clone();
            let milk_foam_container_clone = self.milk_foam_container.clone();
            let water_container_clone = self.water_container.clone();
            let cocoa_container_clone = self.cocoa_container.clone();
            self.dispenser_vec.push(thread::spawn(move || {
                println!("[dispenser {i}] turned on");
                let dispenser = Dispenser::new(
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
    }
}
