use super::order::Order;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct OrderSystem {
    order_queue: VecDeque<Order>,
    finished_queueing: bool,
    busy: bool,
}

impl OrderSystem {
    pub fn new() -> Self {
        Self {
            order_queue: VecDeque::new(),
            finished_queueing: false,
            busy: false,
        }
    }

    pub fn is_busy(&self) -> bool {
        self.busy
    }

    pub fn set_busy(&mut self, state: bool) {
        self.busy = state;
    }

    pub fn save_order(&mut self, order: Order) {
        self.order_queue.push_back(order);
    }

    pub fn set_finished_queueing(&mut self) {
        self.finished_queueing = true;
    }

    pub fn there_are_orders_left(&self) -> bool {
        !(self.finished_queueing && self.order_queue.is_empty())
    }

    pub fn get_order(&mut self) -> Option<Order> {
        self.order_queue.pop_front()
    }

    pub fn queue_is_empty(&self) -> bool {
        self.order_queue.is_empty()
    }

    pub fn finished_queueing(&self) -> bool {
        self.finished_queueing
    }

    #[allow(dead_code)]
    pub fn amount_left_orders(&self) -> usize {
        self.order_queue.len()
    }
}
