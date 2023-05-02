use crate::{
    coffee_maker_components::{
        dispenser::Dispenser, network_rechargeable_container::NetworkRechargeableContainer,
        rechargeable_container::RechargeableContainer,
        unrechargeable_container::UnrechargeableContainer,
    },
    order_management::order_system::OrderSystem,
    statistics_checker::statistic::Statistic,
};
use std::{
    sync::{Arc, Condvar, Mutex},
    thread::{self, JoinHandle},
};

use super::{
    configuration::{CoffeeMakerConfiguration, SecureCounter, SecurePowerState},
    container::Container,
    container_rechargeable_controller::ContainerRechargerController,
    provider_container::ProviderContainer,
};

fn start_dispenser(
    dispenser_number: u32,
    order_queue_monitor: Arc<(Mutex<OrderSystem>, Condvar)>,
    prepared_orders_monitor: Arc<(Mutex<SecureCounter>, Condvar)>,
    ground_coffee_container: Arc<RechargeableContainer>,
    milk_foam_container: Arc<RechargeableContainer>,
    cocoa_container: Arc<UnrechargeableContainer>,
    water_container: Arc<NetworkRechargeableContainer>,
) -> JoinHandle<u32> {
    thread::spawn(move || {
        let dispenser = Dispenser::new();
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
                &prepared_orders_monitor_clone,
            ) {
                finish_processing_orders = finished_processing;
            } else {
                finish_processing_orders = true
            }
        }
        dispenser_number
    })
}

pub struct CoffeeMachine {
    grain_container: Arc<ProviderContainer>,
    milk_container: Arc<ProviderContainer>,
    ground_coffee_container: Arc<RechargeableContainer>,
    milk_foam_container: Arc<RechargeableContainer>,
    cocoa_container: Arc<UnrechargeableContainer>,
    water_container: Arc<NetworkRechargeableContainer>,
    prepared_orders_monitor: Arc<(Mutex<SecureCounter>, Condvar)>,
    dispenser_amount: u32,
    power_monitor: Arc<(Mutex<SecurePowerState>, Condvar)>,
}

impl CoffeeMachine {
    pub fn new(configuration: CoffeeMakerConfiguration) -> Self {
        let grain_container = Arc::new(ProviderContainer::new(
            configuration.grain_capacity,
            configuration.amount_percentage_alert,
            String::from("Coffee grain container"),
        ));
        let milk_container = Arc::new(ProviderContainer::new(
            configuration.milk_capacity,
            configuration.amount_percentage_alert,
            String::from("Milk container"),
        ));
        let ground_coffee_container = Arc::new(RechargeableContainer::new(
            configuration.ground_coffee_capacity,
            String::from("Ground coffee container"),
            configuration.amount_percentage_alert,
            ContainerRechargerController::new(grain_container.clone()),
            configuration.coffee_ground_recharge_rate,
        ));
        let milk_foam_container = Arc::new(RechargeableContainer::new(
            configuration.milk_foam_capacity,
            String::from("Milk foam container"),
            configuration.amount_percentage_alert,
            ContainerRechargerController::new(milk_container.clone()),
            configuration.milk_foam_recharge_rate,
        ));
        let cocoa_container = Arc::new(UnrechargeableContainer::new(
            configuration.cocoa_capacity,
            configuration.amount_percentage_alert,
            String::from("Cocoa container"),
        ));
        let water_container = Arc::new(NetworkRechargeableContainer::new(
            configuration.water_capacity,
            configuration.amount_percentage_alert,
            configuration.heated_water_recharge_rate,
            String::from("Water container"),
        ));
        Self {
            grain_container,
            milk_container,
            ground_coffee_container,
            milk_foam_container,
            cocoa_container,
            water_container,
            prepared_orders_monitor: Arc::new((
                Mutex::new(SecureCounter {
                    busy: false,
                    amount: 0,
                }),
                Condvar::new(),
            )),
            dispenser_amount: configuration.dispenser_amount,
            power_monitor: Arc::new((
                Mutex::new(SecurePowerState {
                    busy: false,
                    on: true,
                }),
                Condvar::new(),
            )),
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
            println!("[dispenser {i}] turned on");
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

    fn turn_off(&self) {
        if let Ok(guard) = self.power_monitor.0.lock() {
            if let Ok(mut power_state) = self.power_monitor.1.wait_while(guard, |state| state.busy)
            {
                power_state.busy = true;
                power_state.on = false;
                power_state.busy = false;
            }
            self.power_monitor.1.notify_all();
        }
    }

    fn is_turned_on(&self) -> bool {
        let mut is_on = true;
        if let Ok(guard) = self.power_monitor.0.lock() {
            if let Ok(mut power_state) = self.power_monitor.1.wait_while(guard, |state| state.busy)
            {
                power_state.busy = true;
                is_on = power_state.on;
                power_state.busy = false;
            }
            self.power_monitor.1.notify_all();
        }
        is_on
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
        self.turn_off();
    }

    pub fn get_statistics(&self) -> (Vec<Statistic>, u32, bool) {
        let statistics_vec = vec![
            self.grain_container.get_statistics(),
            self.milk_container.get_statistics(),
            self.milk_foam_container.get_statistics(),
            self.ground_coffee_container.get_statistics(),
            self.cocoa_container.get_statistics(),
            self.water_container.get_statistics(),
        ];
        let amount_drinks_prepared = self.get_amount_drinks_prepared();

        let is_turned_on = self.is_turned_on();

        (statistics_vec, amount_drinks_prepared, is_turned_on)
    }

    pub fn get_amount_drinks_prepared(&self) -> u32 {
        let mut amount = 0;

        if let Ok(guard) = self.prepared_orders_monitor.0.lock() {
            if let Ok(mut secure_order_counter) = self
                .prepared_orders_monitor
                .1
                .wait_while(guard, |state| state.busy)
            {
                secure_order_counter.busy = true;
                amount = secure_order_counter.amount;
                secure_order_counter.busy = false;
            }
        }
        self.prepared_orders_monitor.1.notify_all();
        amount
    }
}

#[cfg(test)]
mod coffee_maker_test {
    use crate::{
        coffee_maker_components::{
            coffee_machine::CoffeeMachine, configuration::CoffeeMakerConfiguration,
            container::MockContainer,
        },
        statistics_checker::statistic::Statistic,
    };

    #[test]
    fn test01_when_getting_statistics_the_result_is_correct() {
        let mut mock = MockContainer::new();
        mock.expect_get_statistics().returning(|| Statistic {
            container: String::from(""),
            amount_consumed: 0,
            amount_left: 1,
        });

        let coffee_maker = CoffeeMachine::new(CoffeeMakerConfiguration {
            grain_capacity: 1,
            ground_coffee_capacity: 1,
            milk_capacity: 1,
            milk_foam_capacity: 1,
            cocoa_capacity: 1,
            water_capacity: 1,
            dispenser_amount: 1,
            coffee_ground_recharge_rate: 1,
            milk_foam_recharge_rate: 1,
            heated_water_recharge_rate: 1,
            amount_percentage_alert: 0.2,
        });

        let statistics = coffee_maker.get_statistics();

        for stat in statistics.0 {
            assert_eq!(stat.amount_consumed, 0);
            assert_eq!(stat.amount_left, 1);
        }

        assert_eq!(statistics.1, 0);
        assert_eq!(statistics.2, true);
    }

    #[test]
    fn test02_when_asking_if_the_coffe_maker_just_instantiated_is_on_returns_true() {
        let coffee_maker = CoffeeMachine::new(CoffeeMakerConfiguration {
            grain_capacity: 1,
            ground_coffee_capacity: 1,
            milk_capacity: 1,
            milk_foam_capacity: 1,
            cocoa_capacity: 1,
            water_capacity: 1,
            dispenser_amount: 1,
            coffee_ground_recharge_rate: 1,
            milk_foam_recharge_rate: 1,
            heated_water_recharge_rate: 1,
            amount_percentage_alert: 0.2,
        });

        assert_eq!(coffee_maker.is_turned_on(), true);
    }

    #[test]
    fn test03_when_asking_if_the_coffe_maker_just_turned_off_is_on_returns_false() {
        let coffee_maker = CoffeeMachine::new(CoffeeMakerConfiguration {
            grain_capacity: 1,
            ground_coffee_capacity: 1,
            milk_capacity: 1,
            milk_foam_capacity: 1,
            cocoa_capacity: 1,
            water_capacity: 1,
            dispenser_amount: 1,
            coffee_ground_recharge_rate: 1,
            milk_foam_recharge_rate: 1,
            heated_water_recharge_rate: 1,
            amount_percentage_alert: 0.2,
        });

        coffee_maker.turn_off();
        assert_eq!(coffee_maker.is_turned_on(), false);
    }
}
