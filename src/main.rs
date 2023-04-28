mod coffee_maker_components;
mod order_management;
mod order_taker_robot;
mod statistics_checker;

use std::env;
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
    let mut dir = env::current_dir().unwrap().into_os_string();

    let coffee_maker = Arc::new(CoffeeMaker::new(G, M, L, E, C, A, N));
    let monitor_pair = Arc::new((Mutex::new(OrderSystem::new()), Condvar::new()));
    let mut robot = Robot::new(String::from("src/order_taker_robot/example.json"));

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

        //TODO: check res. Poner en el informe que en un principio pense en hacer de a una y despues cambie y los motivos
        match robot.take_orders() {
            Ok(orders) => { 
                robot.queue_orders(orders, &monitor_pair_clone_robot);
            }, 
            Err(msg) => {
                //TODO: frenar todo
            }
        }
        
    });

    coffe_make_handle.join().unwrap();
    order_preparation_handle.join().unwrap();
    statistics_handle.join().unwrap();
    //TODO: cambiar los unwrapps
}
