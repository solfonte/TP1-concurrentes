use crate::order::{order::Order, self};
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};
use std_semaphore::Semaphore;
//capaz no tienen que ser ARC los atributos, sino solo el coffee maker
pub struct CoffeeMaker {
    grain_container: Arc<Mutex<u32>>,
    ground_coffee_container: Arc<Mutex<u32>>,
    milk_container: Arc<Mutex<u32>>,
    milk_foam_container: Arc<Mutex<u32>>,
    cocoa_container: Arc<Mutex<u32>>,
    water_container: Arc<Mutex<u32>>,
    dispenser_semaphore: Arc<Semaphore>,
    dispenser_amount: u32,
    dispenser_vec: Vec<JoinHandle<()>>,
}

impl CoffeeMaker {
    pub fn new(g: u32, m: u32, l: u32, e: u32, c: u32, a: u32, n: u32) -> Self {
        Self {
            grain_container: Arc::new(Mutex::new(g)),
            ground_coffee_container: Arc::new(Mutex::new(m)),
            milk_container: Arc::new(Mutex::new(l)),
            milk_foam_container: Arc::new(Mutex::new(e)),
            cocoa_container: Arc::new(Mutex::new(c)),
            water_container: Arc::new(Mutex::new(a)),
            dispenser_amount: n,
            dispenser_semaphore: Arc::new(Semaphore::new(n as isize)),
            dispenser_vec: Vec::new(),
        }
    }

    pub fn turn_on(&mut self, order_queue_mutex: Arc<Mutex<VecDeque<Order>>>, order_queue_semaphore: Arc<Semaphore<>>) {
        
        for i in 0..self.dispenser_amount {
            let order_queue_mutex_clone = order_queue_mutex.clone();
            let order_queue_semaphore_clone = order_queue_semaphore.clone();
            self.dispenser_vec.push(thread::spawn(move || {
                println!("[dispenser {i}] turned on");
                loop {
                    order_queue_semaphore_clone.acquire();
                    if let Ok(mut order_queue) = order_queue_mutex_clone.lock() {
                        let order = order_queue.pop_front().unwrap();
                        println!("[dispenser number {i}] Order number {:?}", order.get_order_number());

                    }
                    
                    //empiezo a ver tema de recursos.
                    //cuando se termina la imprimo.
                }
            }))
        }
    }
}
