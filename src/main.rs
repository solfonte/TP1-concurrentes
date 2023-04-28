mod coffee_maker_components;
mod order_management;
mod order_taker_robot;
mod statistics_checker;


use std::sync::{Condvar, Mutex};
use std::time::Duration;
use std::{sync::Arc, thread};
use coffee_maker_components::coffee_maker::CoffeeMaker;
use order_management::order_system::OrderSystem;
use order_taker_robot::robot::Robot;
use statistics_checker::statistic_checker::StatisticsChecker;


//TODO: read from file
const N: u32 = 10;
const G: u32 = 30;
const M: u32 = 30;
const L: u32 = 30;
const E: u32 = 30;
const C: u32 = 30;
const A: u32 = 30;

fn main() {
    let coffee_maker = Arc::new(CoffeeMaker::new(G, M, L, E, C, A, N));
    let monitor_pair = Arc::new((Mutex::new(OrderSystem::new()), Condvar::new()));
    let mut robot = Robot::new();

    let monitor_pair_clone = monitor_pair.clone();
    let coffee_maker_clone = coffee_maker.clone();

    let coffe_make_handle = thread::spawn(move || {
        coffee_maker_clone.turn_on(monitor_pair_clone);
    });

    let statistics_handle = thread::spawn(move || {
        let statistics_checker = StatisticsChecker::new(coffee_maker);

        let mut continue_printing_statistics = true;

        while continue_printing_statistics {
            thread::sleep(Duration::from_millis(5));
            continue_printing_statistics = statistics_checker.print_statistics();
            //El sistema debe alertar por consola cuando los contenedores de granos, leche y cacao se encuentran por debajo de X% de capacidad.
        }
    });

    let order_preparation_handle = thread::spawn(move || {
        let mut there_are_orders_left = true;

        let monitor_pair_clone_robot = monitor_pair.clone();

        while there_are_orders_left {
            let result = robot.take_order();

            if let Some(order) = result {
                robot.queue_order(Some(order), &monitor_pair_clone_robot);
            } else {
                robot.queue_order(None, &monitor_pair_clone_robot);
                there_are_orders_left = false;
            }
        }
    });

    coffe_make_handle.join().unwrap();
    order_preparation_handle.join().unwrap();
    statistics_handle.join().unwrap();
    //TODO: cambiar los unwrapps
}
