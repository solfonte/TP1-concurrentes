use std::{sync::{Mutex, Arc}, collections::VecDeque};
use std_semaphore::Semaphore;
use crate::order::order::Order;


pub struct Robot {}

impl Robot {
    pub fn new() -> Self {
        Self {}
    }

    pub fn take_order(&self, order_number: u32) -> Order {
        let order = "10;9;3;5";
        let mut order_vec = Vec::new();
        for j in order.split(";") {
            order_vec.push(j.parse::<u32>().unwrap())
        }
    
        Order::new(
            order_number,
            order_vec[0],
            order_vec[1],
            order_vec[2],
            order_vec[3],
        )
    }

    pub fn queue_order(
        &self,
        order: Order,
        order_queue_mutex: Arc<Mutex<VecDeque<Order>>>,
        order_semaphore: Arc<Semaphore>,
    ) {
        if let Ok(mut order_queue) = order_queue_mutex.lock()
        /* TODO: cambiar el unwrapp */
        {
            order_queue.push_back(order);
        }
        order_semaphore.release();
    }
}