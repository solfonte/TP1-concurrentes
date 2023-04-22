use std::sync::{Condvar, Mutex};
use std::{sync::Arc, thread};
use tp1::coffee_maker::coffee_maker::CoffeeMaker;
use tp1::order::order_system::OrderSystem;
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
    let monitor_pair = Arc::new((Mutex::new(OrderSystem::new()), Condvar::new()));
    let mut robot = Robot::new();

    let monitor_pair_clone = monitor_pair.clone();
    let coffe_make_handle = thread::spawn(move || {
        coffee_maker.turn_on(monitor_pair_clone);
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

    /* let statistics_handle = thread::spawn(move || {

        loop {
            thread::sleep(Duration::from_millis(10));
            //niveles de todos los contenedores
            //cantidad total de bebidas preparadas
            //cantidad total de ingredientes consumidos.


            //El sistema debe alertar por consola cuando los contenedores de granos, leche y cacao se encuentran por debajo de X% de capacidad.

        }

    });*/

    coffe_make_handle.join().unwrap();
    order_preparation_handle.join().unwrap();
}
