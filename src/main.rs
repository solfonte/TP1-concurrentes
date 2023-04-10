use std::collections::VecDeque;
use std::sync::Mutex;
use std::{sync::Arc, thread};
use std_semaphore::Semaphore;
use tp1::coffee_maker::coffee_maker::CoffeeMaker;
use tp1::robot::robot::Robot;

//TODO: read from file
const N: u32 = 10;
const G: u32 = 30;
const M: u32 = 30;
const L: u32 = 30;
const E: u32 = 30;
const C: u32 = 30;
const A: u32 = 30;


fn main() {
    let mut coffee_maker = CoffeeMaker::new(G, M, L, E, C, A, N);
    let robot = Robot::new();

    let order_queue = VecDeque::new();
    let order_queue_mutex = Arc::new(Mutex::new(order_queue));
    let order_semaphore = Arc::new(Semaphore::new(0));

    coffee_maker.turn_on(order_queue_mutex.clone(), order_semaphore.clone());

    let order_preparation_handle = thread::spawn(move || {
        for i in 0..20 {
            let order = robot.take_order(i);
            let order_queue_mutex_clone = order_queue_mutex.clone();
            let order_semaphore_clone = order_semaphore.clone();
            robot.queue_order(order, order_queue_mutex_clone, order_semaphore_clone);
        }
    });

    order_preparation_handle.join().unwrap();
}
