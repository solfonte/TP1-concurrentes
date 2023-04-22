use std_semaphore::Semaphore;

use crate::{order::order::Order, order::order_system::OrderSystem};
use std::{
    sync::{Arc, Mutex, Condvar}};

pub struct Robot {
    vector_pos: usize,
    vector: Vec<String>,
    
}

impl Robot {
    pub fn new() -> Self {
        Self {
            vector_pos: 0,
            vector: Vec::from([String::from("10;9;3;5"), String::from("10;9;3;5"), String::from("10;9;3;5"), String::from("10;9;3;5")])
        }
    }

    pub fn take_order(&mut self) -> Option<Order> {

        if self.vector_pos < self.vector.len() {
            let mut order_vec = Vec::new();
            for j in self.vector[self.vector_pos].split(";") {
                order_vec.push(j.parse::<u32>().unwrap())
            }
            let order_number = self.vector_pos;
            self.vector_pos += 1;
    
            Some(Order::new(
                order_number as u32,
                order_vec[0],
                order_vec[1],
                order_vec[2],
                order_vec[3],
            ))

        } else {
            None
        }
    }

    pub fn queue_order(
        &self,
        order: Option<Order>,
        order_queue_monitor: &Arc<(Mutex<OrderSystem>, Condvar)>
    ) {
        if let Ok(guard) = order_queue_monitor.0.lock() {
            if let Ok(mut order_system) = order_queue_monitor.1.wait_while(guard, |state| {
                state.is_busy()
            }){
                if let Some(_order) = order {
                    //println!("{:?}", _order);
                    order_system.save_order(_order);
                } else {
                    order_system.set_finished_queueing();
                }
            }
        }
        order_queue_monitor.1.notify_all();
    }     
}
