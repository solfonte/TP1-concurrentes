use crate::{
    coffee_maker::{
        dispenser::Dispenser, network_rechargeable_container::NetworkRechargeableContainer,
        rechargeable_container::RechargeableContainer,
        unrechargeable_container::UnrechargeableContainer,
    },
    order::order_system::OrderSystem, statistics_checker::statistic::Statatistic,
};
use std::{
    sync::{Arc, Condvar, Mutex},
    thread::{self, JoinHandle},
};

const COFFEE_RECHARGING_RATE: u32 = 10;
const FOAM_RECHARGING_RATE: u32 = 5;

use super::{
    container_rechargeable_controller::ContainerRechargerController,
    provider_container::ProviderContainer, container::Container,
};
//capaz no tienen que ser ARC los atributos, sino solo el coffee maker

fn start_dispenser(
    dispenser_number: u32,
    order_queue_monitor: Arc<(Mutex<OrderSystem>, Condvar)>,
    prepared_orders_monitor: Arc<(Mutex<(bool, u32)>, Condvar)>,
    ground_coffee_container: Arc<RechargeableContainer>,
    milk_foam_container: Arc<RechargeableContainer>,
    cocoa_container: Arc<UnrechargeableContainer>,
    water_container: Arc<NetworkRechargeableContainer>,
) -> JoinHandle<u32> {
    thread::spawn(move || {
        //println!("[dispenser {dispenser_number}] turned on");
        let dispenser = Dispenser::new(dispenser_number);
        let ground_coffee_container_clone = ground_coffee_container.clone();
        let milk_foam_container_clone = milk_foam_container.clone();
        let water_container_clone = water_container.clone();
        let cocoa_container = cocoa_container.clone();
        let prepared_orders_monitor_clone = prepared_orders_monitor.clone();

        let mut finish_processing_orders: bool = false;
        while !finish_processing_orders {
            if let Ok(finished_processing) = dispenser.process_order(
                &ground_coffee_container_clone,
                &milk_foam_container_clone,
                &water_container_clone,
                &cocoa_container,
                &order_queue_monitor,
                &prepared_orders_monitor_clone
            ) {
                finish_processing_orders = finished_processing;
            } else {
                /*TODO: condicion de error */
                finish_processing_orders = true
            }
        }

        dispenser_number
    })
}

pub struct CoffeeMaker {
    grain_container: Arc<ProviderContainer>,
    milk_container: Arc<ProviderContainer>,
    ground_coffee_container: Arc<RechargeableContainer>,
    milk_foam_container: Arc<RechargeableContainer>,
    cocoa_container: Arc<UnrechargeableContainer>,
    water_container: Arc<NetworkRechargeableContainer>,
    prepared_orders_monitor: Arc<(Mutex<(bool, u32)>, Condvar)>, 
    dispenser_amount: u32,
}

impl CoffeeMaker {
    pub fn new(g: u32, m: u32, l: u32, e: u32, c: u32, a: u32, n: u32) -> Self {
        let grain_container = Arc::new(ProviderContainer::new(
            g,
            String::from("granos"),
        ));
        let milk_container = Arc::new(ProviderContainer::new(
            l,
            String::from("milk"),
        ));
        let ground_coffee_container = Arc::new(RechargeableContainer::new(
            m,
            String::from("cafe"),
            ContainerRechargerController::new(grain_container.clone()),
            COFFEE_RECHARGING_RATE,
        ));
        let milk_foam_container = Arc::new(RechargeableContainer::new(
            e,
            String::from("espuma"),
            ContainerRechargerController::new(milk_container.clone()),
            FOAM_RECHARGING_RATE,
        ));
        let cocoa_container = Arc::new(UnrechargeableContainer::new(
            c,
            String::from("cacao"),
        ));
        let water_container = Arc::new(NetworkRechargeableContainer::new(
            a,
            String::from("agua"),
        ));
        Self {
            grain_container,
            milk_container,
            ground_coffee_container,
            milk_foam_container,
            cocoa_container,
            water_container,
            prepared_orders_monitor: Arc::new((Mutex::new((false, 0)), Condvar::new())),
            dispenser_amount: n,
        }
    }

    fn turn_dispensers_on(
        &self,
        ground_coffee_container: Arc<RechargeableContainer>,
        milk_foam_container: Arc<RechargeableContainer>,
        cocoa_container: Arc<UnrechargeableContainer>,
        water_container: Arc<NetworkRechargeableContainer>,
        order_queue_monitor: &Arc<(Mutex<OrderSystem>, Condvar)>,
    ) -> Vec<JoinHandle<u32>> {
        let mut dispenser_handles = Vec::new();
        for i in 0..self.dispenser_amount {
            let dispenser_handle = start_dispenser(
                i,
                order_queue_monitor.clone(),
                self.prepared_orders_monitor.clone(),
                ground_coffee_container.clone(),
                milk_foam_container.clone(),
                cocoa_container.clone(),
                water_container.clone(),
            );
            dispenser_handles.push(dispenser_handle)
        }
        dispenser_handles
    }

    pub fn turn_on(&self, order_queue_monitor: Arc<(Mutex<OrderSystem>, Condvar)>) {

        let dispenser_handles = self.turn_dispensers_on(
            self.ground_coffee_container.clone(),
            self.milk_foam_container.clone(),
            self.cocoa_container.clone(),
            self.water_container.clone(),
            &order_queue_monitor,
        );

        for d_handle in dispenser_handles {
            if let Ok(dispenser_number) = d_handle.join() {
                println!("[dispenser {}] turned off", dispenser_number);
            }
        }
    }

    pub fn get_cocoa_statistics(&self) -> Statatistic {
        self.cocoa_container.get_statistics()
    }

    pub fn get_coffee_grain_statistics(&self) -> Statatistic {
        self.grain_container.get_statistics()
    }

    pub fn get_ground_coffee_statistics(&self) -> Statatistic {
        self.ground_coffee_container.get_statistics()
    }

    pub fn get_milk_statistics(&self) -> Statatistic {
        self.milk_container.get_statistics()
    }

    pub fn get_water_statistics(&self) -> Statatistic {
        self.water_container.get_statistics()
    }

    pub fn get_milk_foam_statistics(&self) -> Statatistic {
        self.milk_foam_container.get_statistics()
    }

    pub fn get_amount_drinks_prepared(&self) -> u32 {

        let mut amount = 0;

        if let Ok(guard) = self.prepared_orders_monitor.0.lock() {
            if let Ok(mut order_system) = self.prepared_orders_monitor
                .1
                .wait_while(guard, |state| state.0)
            {
                order_system.0 = true;
                amount = order_system.1;
                order_system.0 = false;
            }
        }
        self.prepared_orders_monitor.1.notify_all();
        amount
    }
}
