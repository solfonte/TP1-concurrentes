mod coffee_maker_components;
mod order_management;
mod order_taker_robot;
mod statistics_checker;

use coffee_maker_components::coffee_machine::CoffeeMachine;
use coffee_maker_components::configuration::{CoffeeMakerConfiguration, ConfigurationReader};
use order_management::order_system::OrderSystem;
use order_taker_robot::robot::Robot;
use statistics_checker::statistic_checker::StatisticsChecker;
use std::env;
use std::sync::{Condvar, Mutex};
use std::time::Duration;
use std::{sync::Arc, thread};

fn validate_arguments() -> Option<(String, String)> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        0 | 1 => {
            println!("[ERROR]: missing arguuments. Remember to run the program with the following arguments:");
            println!("Cargo run -- <orders-file-path> <config-file>");
            None
        }
        2 => {
            if args[1] == "help" {
                println!("To run the program with the following arguments:");
                println!("Cargo run -- <orders-file-path> <config-file>");
            }
            None
        }
        3 => Some((args[1].clone(), args[2].clone())),
        _ => None,
    }
}

fn main() {
    let orders_file: String;
    let configs_file: String;

    match validate_arguments() {
        Some(files_pair) => {
            orders_file = files_pair.0;
            configs_file = files_pair.1;
        }
        None => {
            return;
        }
    }
    let configuration_reader = ConfigurationReader::new(configs_file);

    let configuration: CoffeeMakerConfiguration = match configuration_reader.read_configuration() {
        Ok(c) => c,
        Err(msg) => {
            println!("[Error] While reading configuration: {}", msg);
            return;
        }
    };

    let coffee_maker = Arc::new(CoffeeMachine::new(configuration));
    let orders_monitor_pair = Arc::new((Mutex::new(OrderSystem::new()), Condvar::new()));
    let mut robot = Robot::new(orders_file);

    let orders_monitor_pair_clone = orders_monitor_pair.clone();
    let coffee_maker_clone = coffee_maker.clone();

    let coffe_make_handle = thread::spawn(move || {
        coffee_maker_clone.turn_on(orders_monitor_pair_clone);
    });

    let coffee_maker_for_statistics_clone = coffee_maker.clone();
    let statistics_handle = thread::spawn(move || {
        let statistics_checker = StatisticsChecker::new(coffee_maker_for_statistics_clone);

        let mut continue_printing_statistics = true;

        while continue_printing_statistics {
            thread::sleep(Duration::from_millis(1000));
            continue_printing_statistics = statistics_checker.print_statistics();
        }
    });

    let order_preparation_handle = thread::spawn(move || {
        let orders_monitor_pair_clone_robot = orders_monitor_pair.clone();

        match robot.take_orders(&orders_monitor_pair_clone_robot) {
            Ok(_) => {}
            Err(error_msg) => {
                println!("[Error while taking the orders]: {}", error_msg);
                coffee_maker.turn_off();
            }
        }
    });

    if coffe_make_handle.join().is_ok() {
        println!("Coffee machine turned off");
    }
    let _ = order_preparation_handle.join();
    let _ = statistics_handle.join();
}
