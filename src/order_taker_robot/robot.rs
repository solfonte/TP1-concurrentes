use serde_json::Deserializer;

use crate::{order_management::order::Order, order_management::order_system::OrderSystem};
use std::{sync::{Arc, Condvar, Mutex}, collections::VecDeque};

use super::file_reader::FileReader;
use serde_json::Value;

pub struct Robot {
    file_reader: FileReader
}

impl Robot {
    pub fn new(file_name: String) -> Self {
        Self {
            file_reader: FileReader::new(file_name)
        }
    }

    pub fn take_orders(&mut self) -> Result<Vec<Order>, String> {
        match self.file_reader.read() {
            Ok(orders_string) => {
                let stream = serde_json::from_str::<Vec<Order>>(&orders_string);
                match stream {
                    Ok(order_queue) => Ok(order_queue),
                    Err(err) => Err(err.to_string())
                } 
            },
            Err(error_msg) => {
                Err(error_msg)
            }
        }

    }

    pub fn queue_orders(
        &self,
        orders: Vec<Order>,
        order_queue_monitor: &Arc<(Mutex<OrderSystem>, Condvar)>,
    ) {
        if let Ok(guard) = order_queue_monitor.0.lock() {
            if let Ok(mut order_system) = order_queue_monitor
                .1
                .wait_while(guard, |state| state.is_busy())
            {
                order_system.set_busy(true);
                for order in orders {
                    order_system.save_order(order);
                }
                order_system.set_busy(false);
            }
        }
        order_queue_monitor.1.notify_all();
    }
}
