use crate::{order_management::order::Order, order_management::order_system::OrderSystem};
use std::sync::{Arc, Condvar, Mutex};

use super::file_reader::FileReader;

pub struct Robot {
    file_reader: FileReader
}

impl Robot {
    pub fn new(file_name: String) -> Self {
        Self {
            file_reader: FileReader::new(file_name)
        }
    }

    pub fn read_orders(&mut self) -> Result<Vec<Order>, String> {

        match self.file_reader.read() {
            Ok(orders_string) => {
                let stream = serde_json::from_str::<Vec<Order>>(&orders_string);
                println!("reading orderrs");
                match stream {
                    Ok(order_queue) => Ok(order_queue),
                    Err(msg) => Err(msg.to_string())
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
                order_system.set_finished_queueing();
                order_system.set_busy(false);
            }
        }
        order_queue_monitor.1.notify_all();
    }

    pub fn take_orders(&mut self, order_queue_monitor: &Arc<(Mutex<OrderSystem>, Condvar)>) -> Result<(), String> {
        match self.read_orders() {
            Ok(orders) => { 
                self.queue_orders(orders, order_queue_monitor);
                Ok(())
            }, 
            Err(error_msg) => {
                Err(error_msg)
            }
        }
    }
}


#[cfg(test)]
mod robot_test {
    use std::{sync::{Arc, Mutex, Condvar}, result};

    use crate::order_management::{order_system::OrderSystem, order::{self, Order}};

    use super::Robot;

    #[test]
    fn test01_when_taking_only_one_order_the_result_is_a_vector_with_one_order() {
        let mut robot = Robot::new(String::from("src/test_order_files/one_order.json"));
        //let orders_monitor_pair = Arc::new((Mutex::new(OrderSystem::new()), Condvar::new()));
        let result = robot.read_orders();
                
        assert!(result.is_ok());

        let orders = result.expect("Vector");

        assert_eq!(orders.len(), 1);
        assert_eq!(orders[0].cocoa_amount, 0);
        assert_eq!(orders[0].coffee_amount, 5);
        assert_eq!(orders[0].milk_foam_amount, 5);
        assert_eq!(orders[0].water_amount, 3);
        assert_eq!(orders[0].order_number, 0);
    }

    #[test]
    fn test02_when_taking_two_orders_the_result_is_a_vector_with_two_orders() {
        let mut robot = Robot::new(String::from("src/test_order_files/two_orders.json"));
        let result = robot.read_orders();
                
        assert!(result.is_ok());

        let orders = result.expect("Vector");

        assert_eq!(orders.len(), 2);
        assert_eq!(orders[0].cocoa_amount, 0);
        assert_eq!(orders[0].coffee_amount, 5);
        assert_eq!(orders[0].milk_foam_amount, 5);
        assert_eq!(orders[0].water_amount, 3);
        assert_eq!(orders[0].order_number, 0);

        assert_eq!(orders[1].cocoa_amount, 1);
        assert_eq!(orders[1].coffee_amount, 1);
        assert_eq!(orders[1].milk_foam_amount, 1);
        assert_eq!(orders[1].water_amount, 1);
        assert_eq!(orders[1].order_number, 1);
    }

    #[test]
    fn test03_when_taking_only_one_order_the_order_is_correctly_queued() {
        let robot = Robot::new(String::from("src/test_order_files/one_order.json"));
        let orders_monitor_pair = Arc::new((Mutex::new(OrderSystem::new()), Condvar::new()));
        let order_vector = vec![Order{order_number: 0, coffee_amount: 5, cocoa_amount: 0, milk_foam_amount: 5, water_amount: 3}];
        let monitor_clone = orders_monitor_pair.clone();
        robot.queue_orders(order_vector, &monitor_clone);
        
        match orders_monitor_pair.0.lock() {
            Ok(mut order_system) => {
                assert_eq!(order_system.amount_left_orders(), 1);

                match order_system.get_order() {
                    Some(order) => {
                        assert_eq!(order.cocoa_amount, 0);
                        assert_eq!(order.coffee_amount, 5);
                        assert_eq!(order.milk_foam_amount, 5);
                        assert_eq!(order.water_amount, 3);
                        assert_eq!(order.order_number, 0);
                    }, 
                    None => {assert!(false)}
                }
            },
            Err(_) => {assert!(false)}
        };

    }

    #[test]
    fn test04_when_taking_only_two_orders_the_orders_are_correctly_queued() {
        let robot = Robot::new(String::from("src/test_order_files/fake.json"));
        let orders_monitor_pair = Arc::new((Mutex::new(OrderSystem::new()), Condvar::new()));
        let order_vector = vec![Order{order_number: 0, coffee_amount: 5, cocoa_amount: 0, milk_foam_amount: 5, water_amount: 3},
                                            Order{order_number: 1, coffee_amount: 1, cocoa_amount: 1, milk_foam_amount: 1, water_amount: 1}];
        let monitor_clone = orders_monitor_pair.clone();
        robot.queue_orders(order_vector, &monitor_clone);
        
        match orders_monitor_pair.0.lock() {
            Ok(mut order_system) => {
                assert_eq!(order_system.amount_left_orders(), 2);
                
                match order_system.get_order() {
                    Some(order) => {
                        assert_eq!(order.cocoa_amount, 0);
                        assert_eq!(order.coffee_amount, 5);
                        assert_eq!(order.milk_foam_amount, 5);
                        assert_eq!(order.water_amount, 3);
                        assert_eq!(order.order_number, 0);
                    }, 
                    None => {assert!(false)}
                }

                match order_system.get_order() {
                    Some(order) => {
                        assert_eq!(order.cocoa_amount, 1);
                        assert_eq!(order.coffee_amount, 1);
                        assert_eq!(order.milk_foam_amount, 1);
                        assert_eq!(order.water_amount, 1);
                        assert_eq!(order.order_number, 1);
                    }, 
                    None => {assert!(false)}
                }
            },
            Err(_) => {assert!(false)}
        };
    }

    #[test]
    fn test06_when_taking_only_one_order_the_order_is_correctly_taken() {
        let mut robot = Robot::new(String::from("src/test_order_files/one_order.json"));
        let orders_monitor_pair = Arc::new((Mutex::new(OrderSystem::new()), Condvar::new()));

        let result = robot.take_orders(&orders_monitor_pair);
        
        assert!(result.is_ok());

        match orders_monitor_pair.0.lock() {
            Ok(mut order_system) => {
                assert_eq!(order_system.amount_left_orders(), 1);

                match order_system.get_order() {
                    Some(order) => {
                        assert_eq!(order.cocoa_amount, 0);
                        assert_eq!(order.coffee_amount, 5);
                        assert_eq!(order.milk_foam_amount, 5);
                        assert_eq!(order.water_amount, 3);
                        assert_eq!(order.order_number, 0);
                    }, 
                    None => {assert!(false)}
                }
            },
            Err(_) => {assert!(false)}
        };
    }

    #[test]
    fn test07_when_taking_two_orders_the_orders_are_correctly_taken() {
        let mut robot = Robot::new(String::from("src/test_order_files/two_orders.json"));
        let orders_monitor_pair = Arc::new((Mutex::new(OrderSystem::new()), Condvar::new()));

        let result = robot.take_orders(&orders_monitor_pair);
        
        assert!(result.is_ok());

        match orders_monitor_pair.0.lock() {
            Ok(mut order_system) => {
                assert_eq!(order_system.amount_left_orders(), 2);
                
                match order_system.get_order() {
                    Some(order) => {
                        assert_eq!(order.cocoa_amount, 0);
                        assert_eq!(order.coffee_amount, 5);
                        assert_eq!(order.milk_foam_amount, 5);
                        assert_eq!(order.water_amount, 3);
                        assert_eq!(order.order_number, 0);
                    }, 
                    None => {assert!(false)}
                }

                match order_system.get_order() {
                    Some(order) => {
                        assert_eq!(order.cocoa_amount, 1);
                        assert_eq!(order.coffee_amount, 1);
                        assert_eq!(order.milk_foam_amount, 1);
                        assert_eq!(order.water_amount, 1);
                        assert_eq!(order.order_number, 1);
                    }, 
                    None => {assert!(false)}
                }
            },
            Err(_) => {assert!(false)}
        };
    }
}


