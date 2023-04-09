use std::collections::VecDeque;
use std::sync::Mutex;
use std::{sync::Arc, thread};
use tp1::coffee_maker::coffee_maker::CoffeeMaker;
use tp1::order::order::Order;
use std_semaphore::Semaphore;

const N: u32 = 10;
const G: u32 = 30;
const M: u32 = 30;
const L: u32 = 30;
const E: u32 = 30;
const C: u32 = 30;
const A: u32 = 30;

fn queue_order(
    order_number: u32,
    coffee_amount: u32,
    cocoa_amount: u32,
    milk_foam_amount: u32,
    water_amount: u32,
    order_queue_mutex: Arc<Mutex<VecDeque<Order>>>,
    order_semaphore: Arc::<Semaphore<>>
) {
    let order = Order::new(
        order_number,
        coffee_amount,
        cocoa_amount,
        milk_foam_amount,
        water_amount,
    );
    println!("{:?}", order);
    if let Ok(mut order_queue) = order_queue_mutex.lock() /* TODO: cambiar el unwrapp */ {
        order_queue.push_back(order);
    }
    order_semaphore.release();
}

fn main() {
    let mut coffee_maker = CoffeeMaker::new(G, M, L, E, C, A, N);

    let order_queue = VecDeque::new();
    let order_queue_mutex = Arc::new(Mutex::new(order_queue));
    let order_semaphore = Arc::new(Semaphore::new(0));
    
    coffee_maker.turn_on(order_queue_mutex.clone(), order_semaphore.clone());
    
    let order_preparation_handle = thread::spawn(move || {
        for i in 0..20 {
            // TODO: get coffee order amounts from file
            let order_queue_mutex_clone = order_queue_mutex.clone();
            let order_semaphore_clone = order_semaphore.clone();
            queue_order(i, 12, 12, 12, 30, order_queue_mutex_clone, order_semaphore_clone);
        }
    });

    order_preparation_handle.join().unwrap();
}
