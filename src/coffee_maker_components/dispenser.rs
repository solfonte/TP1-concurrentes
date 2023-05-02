use super::{
    configuration::SecureCounter, network_rechargeable_container::NetworkRechargeableContainer,
    rechargeable_container::RechargeableContainer,
    unrechargeable_container::UnrechargeableContainer,
};
use crate::{
    coffee_maker_components::container::Container,
    order_management::{order::Order, order_system::OrderSystem},
};
use std::time::Duration;
use std::{
    sync::{Arc, Condvar, Mutex},
    thread,
};

pub struct Dispenser {}

impl Dispenser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn dispense_resource<T: Container>(
        &self,
        ingredient_amount: u32,
        container: &T,
    ) -> Result<u32, &str> {
        let extraction_result = container.extract(ingredient_amount);
        match extraction_result {
            Ok(ingredient_taken) => {
                thread::sleep(Duration::from_millis(20 * ingredient_amount as u64));
                Ok(ingredient_taken)
            }
            Err(msg) => {
                println!("[Error extracting]{msg}");
                Err("")
            }
        }
    }

    pub fn add_prepared_order(
        &self,
        prepared_orders_monitor: &Arc<(Mutex<SecureCounter>, Condvar)>,
    ) {
        if let Ok(guard) = prepared_orders_monitor.0.lock() {
            if let Ok(mut secured_order_counter) = prepared_orders_monitor
                .1
                .wait_while(guard, |state| state.busy)
            {
                secured_order_counter.busy = true;
                secured_order_counter.amount += 1;
                secured_order_counter.busy = false;
            }
        }
        prepared_orders_monitor.1.notify_all();
    }

    pub fn prepare_order(
        &self,
        order: Order,
        coffee_container: &RechargeableContainer,
        foam_container: &RechargeableContainer,
        water_container: &NetworkRechargeableContainer,
        cocoa_container: &UnrechargeableContainer,
    ) -> Result<u32, String> {
        let mut ingredient_not_available = false;

        let coffee_amount_required = order.get_coffee_amount();
        if coffee_amount_required > 0 {
            let coffee_result = self.dispense_resource(coffee_amount_required, coffee_container);
            match coffee_result {
                Ok(amount) => {
                    if amount == 0 {
                        ingredient_not_available = true;
                    }
                }
                Err(msg) => {
                    return Err(String::from(msg));
                }
            }
        }

        let cocoa_amount_required = order.get_cocoa_amount();
        if cocoa_amount_required > 0 {
            let cocoa_result = self.dispense_resource(cocoa_amount_required, cocoa_container);
            match cocoa_result {
                Ok(amount) => {
                    if amount == 0 {
                        ingredient_not_available = true;
                    }
                }
                Err(msg) => {
                    return Err(String::from(msg));
                }
            }
        }

        let milk_foam_amount_required = order.get_milk_foam_amount();
        if milk_foam_amount_required > 0 {
            let foam_result = self.dispense_resource(milk_foam_amount_required, foam_container);
            match foam_result {
                Ok(amount) => {
                    if amount == 0 {
                        ingredient_not_available = true;
                    }
                }
                Err(msg) => {
                    return Err(String::from(msg));
                }
            }
        }

        let water_amount_required = order.get_water_amount();
        if water_amount_required > 0 {
            let water_result = self.dispense_resource(water_amount_required, water_container);
            match water_result {
                Ok(amount) => {
                    if amount == 0 {
                        ingredient_not_available = true;
                    }
                }
                Err(msg) => {
                    return Err(String::from(msg));
                }
            }
        }

        let mut amount_orders_prepared = 0;
        if !ingredient_not_available {
            println!("[FINISHED] ORDER {}", order.get_order_number());
            amount_orders_prepared = 1;
        }
        Ok(amount_orders_prepared)
    }

    pub fn take_order_from_queue(
        &self,
        order_queue_monitor: &Arc<(Mutex<OrderSystem>, Condvar)>,
    ) -> Option<Order> {
        let order;
        let mut result = None;

        if let Ok(guard) = order_queue_monitor.0.lock() {
            if let Ok(mut order_system) = order_queue_monitor.1.wait_while(guard, |state| {
                state.is_busy() || (state.queue_is_empty() && !state.finished_queueing())
            }) {
                order_system.set_busy(true);
                if !order_system.there_are_orders_left() {
                    result = None;
                } else if let Some(_order) = order_system.get_order() {
                    order = _order;
                    result = Some(order);
                }
                order_system.set_busy(false);
            }
        }
        order_queue_monitor.1.notify_all();
        result
    }

    pub fn process_order(
        &self,
        coffee_container: &RechargeableContainer,
        foam_container: &RechargeableContainer,
        water_container: &NetworkRechargeableContainer,
        cocoa_container: &UnrechargeableContainer,
        order_queue_monitor: &Arc<(Mutex<OrderSystem>, Condvar)>,
        prepared_orders_monitor: &Arc<(Mutex<SecureCounter>, Condvar)>,
    ) -> Result<bool, String> {
        if let Some(order) = self.take_order_from_queue(order_queue_monitor) {
            let result = self.prepare_order(
                order,
                coffee_container,
                foam_container,
                water_container,
                cocoa_container,
            );

            match result {
                Ok(order_prepared) => {
                    if order_prepared == 1 {
                        self.add_prepared_order(prepared_orders_monitor);
                    }
                    return Ok(false);
                }
                Err(msg) => return Err(msg),
            }
        }
        Ok(true)
    }
}

#[cfg(test)]
mod dispenser_test {

    use crate::coffee_maker_components;
    use crate::coffee_maker_components::dispenser::Dispenser;
    use crate::coffee_maker_components::network_rechargeable_container::NetworkRechargeableContainer;
    use crate::coffee_maker_components::rechargeable_container::RechargeableContainer;
    use crate::coffee_maker_components::unrechargeable_container::UnrechargeableContainer;
    use crate::coffee_maker_components::{
        container_rechargeable_controller::ContainerRechargerController,
        provider_container::ProviderContainer,
    };
    use crate::order_management::order::Order;
    use coffee_maker_components::container::Container;
    use std::sync::Arc;

    #[test]
    fn test01_when_dispensing_three_units_from_unrechargeable_container_which_are_available_the_dispenser_returns_three_units(
    ) {
        let units = 3;

        let container = UnrechargeableContainer::new(10, 0.2, String::from("container"));
        let dispenser = Dispenser::new();

        let dispenser_result = dispenser.dispense_resource(units, &container);

        assert_eq!(dispenser_result, Ok(units));
    }

    #[test]
    fn test02_when_dispensing_three_units_from_rechargeable_container_which_are_available_the_dispenser_returns_three_units(
    ) {
        let units = 3;

        let container = RechargeableContainer::new(
            10,
            String::from("container"),
            0.2,
            ContainerRechargerController::new(Arc::new(ProviderContainer::new(
                0,
                0.2,
                String::from("provider"),
            ))),
            1,
        );
        let dispenser = Dispenser::new();

        let dispenser_result = dispenser.dispense_resource(units, &container);

        assert_eq!(dispenser_result, Ok(units));
    }

    #[test]
    fn test03_when_dispensing_three_units_from_network_rechargeable_container_which_are_available_the_dispenser_returns_three_units(
    ) {
        let units = 3;

        let container = NetworkRechargeableContainer::new(10, 0.2, 2, String::from("container"));
        let dispenser = Dispenser::new();

        let dispenser_result = dispenser.dispense_resource(units, &container);

        assert_eq!(dispenser_result, Ok(units));
    }

    #[test]
    fn test04_when_dispensing_three_units_from_unrechargeable_container_which_are_not_available_the_dispenser_returns_cero_units(
    ) {
        let units = 3;

        let container = UnrechargeableContainer::new(5, 0.2, String::from("container"));
        let dispenser = Dispenser::new();
        let _ = container.extract(4);
        let dispenser_result = dispenser.dispense_resource(units, &container);

        assert_eq!(dispenser_result, Ok(0));
    }

    #[test]
    fn test05_when_dispensing_three_units_from_rechargeable_container_which_are_not_available_and_cannot_be_recharged_the_dispenser_returns_cero_units(
    ) {
        let units = 3;

        let container = RechargeableContainer::new(
            5,
            String::from("container"),
            0.2,
            ContainerRechargerController::new(Arc::new(ProviderContainer::new(
                0,
                0.2,
                String::from("provider"),
            ))),
            1,
        );
        let _ = container.extract(4);
        let dispenser = Dispenser::new();
        let dispenser_result = dispenser.dispense_resource(units, &container);
        assert_eq!(dispenser_result, Ok(0));
    }

    #[test]
    fn test06_when_dispensing_three_units_from_network_rechargeable_container_which_are_not_available_but_is_below_max_capacity_the_dispenser_returns_three_units(
    ) {
        let units = 3;

        let container = NetworkRechargeableContainer::new(5, 0.2, 2, String::from("container"));
        let dispenser = Dispenser::new();
        let _ = container.extract(4);
        let dispenser_result = dispenser.dispense_resource(units, &container);

        assert_eq!(dispenser_result, Ok(units));
    }

    #[test]
    fn test07_when_dispensing_three_units_from_rechargeable_container_which_are_available_and_can_be_recharged_the_dispenser_returns_three_units(
    ) {
        let units = 3;

        let container = RechargeableContainer::new(
            5,
            String::from("container"),
            0.2,
            ContainerRechargerController::new(Arc::new(ProviderContainer::new(
                5,
                0.2,
                String::from("provider"),
            ))),
            1,
        );
        let _ = container.extract(4);
        let dispenser = Dispenser::new();
        let dispenser_result = dispenser.dispense_resource(units, &container);
        assert_eq!(dispenser_result, Ok(3));
    }

    #[test]
    fn test08_when_preparing_an_order_and_all_ingredients_are_available_the_dispenser_result_is_one(
    ) {
        let coffee_amount = 3;
        let cocoa_amount = 3;
        let milk_foam_amount = 3;
        let water_amount = 3;

        let grain_container = ProviderContainer::new(10, 0.2, String::from("grain container"));
        let coffee_container = RechargeableContainer::new(
            5,
            String::from("coffee container"),
            0.2,
            ContainerRechargerController::new(Arc::new(grain_container)),
            1,
        );

        let milk_container = ProviderContainer::new(10, 0.2, String::from("milk container"));
        let foam_container = RechargeableContainer::new(
            5,
            String::from("foam container"),
            0.2,
            ContainerRechargerController::new(Arc::new(milk_container)),
            1,
        );
        let cocoa_container = UnrechargeableContainer::new(5, 0.2, String::from("container"));
        let water_container =
            NetworkRechargeableContainer::new(5, 0.2, 2, String::from("container"));

        let dispenser = Dispenser::new();
        let dispenser_result = dispenser.prepare_order(
            Order {
                order_number: 1,
                coffee_amount,
                cocoa_amount,
                milk_foam_amount,
                water_amount,
            },
            &coffee_container,
            &foam_container,
            &water_container,
            &cocoa_container,
        );
        assert_eq!(dispenser_result, Ok(1));
    }

    #[test]
    fn test09_when_preparing_an_order_and_all_ingredients_except_from_cocoa_are_available_the_dispenser_result_is_cero(
    ) {
        let coffee_amount = 3;
        let cocoa_amount = 3;
        let milk_foam_amount = 3;
        let water_amount = 3;

        let grain_container = ProviderContainer::new(10, 0.2, String::from("grain container"));
        let coffee_container = RechargeableContainer::new(
            5,
            String::from("coffee container"),
            0.2,
            ContainerRechargerController::new(Arc::new(grain_container)),
            1,
        );

        let milk_container = ProviderContainer::new(10, 0.2, String::from("milk container"));
        let foam_container = RechargeableContainer::new(
            5,
            String::from("foam container"),
            0.2,
            ContainerRechargerController::new(Arc::new(milk_container)),
            1,
        );
        let cocoa_container = UnrechargeableContainer::new(5, 0.2, String::from("container"));
        let _ = cocoa_container.extract(5);
        let water_container =
            NetworkRechargeableContainer::new(5, 0.2, 2, String::from("container"));

        let dispenser = Dispenser::new();
        let dispenser_result = dispenser.prepare_order(
            Order {
                order_number: 1,
                coffee_amount,
                cocoa_amount,
                milk_foam_amount,
                water_amount,
            },
            &coffee_container,
            &foam_container,
            &water_container,
            &cocoa_container,
        );
        assert_eq!(dispenser_result, Ok(0));
    }

    #[test]
    fn test10_when_preparing_an_order_and_all_ingredients_except_from_coffee_which_cannot_be_recharged_are_available_the_dispenser_result_is_cero(
    ) {
        let coffee_amount = 3;
        let cocoa_amount = 3;
        let milk_foam_amount = 3;
        let water_amount = 3;

        let grain_container = ProviderContainer::new(0, 0.2, String::from("grain container"));
        let coffee_container = RechargeableContainer::new(
            5,
            String::from("coffee container"),
            0.2,
            ContainerRechargerController::new(Arc::new(grain_container)),
            1,
        );
        let _ = coffee_container.extract(5);

        let milk_container = ProviderContainer::new(10, 0.2, String::from("milk container"));
        let foam_container = RechargeableContainer::new(
            5,
            String::from("foam container"),
            0.2,
            ContainerRechargerController::new(Arc::new(milk_container)),
            1,
        );
        let cocoa_container = UnrechargeableContainer::new(5, 0.2, String::from("container"));
        let water_container =
            NetworkRechargeableContainer::new(5, 0.2, 2, String::from("container"));

        let dispenser = Dispenser::new();
        let dispenser_result = dispenser.prepare_order(
            Order {
                order_number: 1,
                coffee_amount,
                cocoa_amount,
                milk_foam_amount,
                water_amount,
            },
            &coffee_container,
            &foam_container,
            &water_container,
            &cocoa_container,
        );
        assert_eq!(dispenser_result, Ok(0));
    }

    #[test]
    fn test11_when_preparing_an_order_and_all_ingredients_except_from_coffee_are_available_the_dispenser_result_is_cero(
    ) {
        let coffee_amount = 3;
        let cocoa_amount = 3;
        let milk_foam_amount = 3;
        let water_amount = 3;

        let grain_container = ProviderContainer::new(10, 0.2, String::from("grain container"));
        let coffee_container = RechargeableContainer::new(
            5,
            String::from("coffee container"),
            0.2,
            ContainerRechargerController::new(Arc::new(grain_container)),
            1,
        );

        let milk_container = ProviderContainer::new(0, 0.2, String::from("milk container"));
        let foam_container = RechargeableContainer::new(
            5,
            String::from("foam container"),
            0.2,
            ContainerRechargerController::new(Arc::new(milk_container)),
            1,
        );
        let _ = foam_container.extract(5);

        let cocoa_container = UnrechargeableContainer::new(5, 0.2, String::from("container"));
        let water_container =
            NetworkRechargeableContainer::new(5, 0.2, 2, String::from("container"));

        let dispenser = Dispenser::new();
        let dispenser_result = dispenser.prepare_order(
            Order {
                order_number: 1,
                coffee_amount,
                cocoa_amount,
                milk_foam_amount,
                water_amount,
            },
            &coffee_container,
            &foam_container,
            &water_container,
            &cocoa_container,
        );
        assert_eq!(dispenser_result, Ok(0));
    }

    #[test]
    fn test12_when_preparing_an_order_and_all_ingredients_except_from_water_which_can_be_recharged_are_available_the_dispenser_result_is_one(
    ) {
        let coffee_amount = 3;
        let cocoa_amount = 3;
        let milk_foam_amount = 3;
        let water_amount = 3;

        let grain_container = ProviderContainer::new(10, 0.2, String::from("grain container"));
        let coffee_container = RechargeableContainer::new(
            5,
            String::from("coffee container"),
            0.2,
            ContainerRechargerController::new(Arc::new(grain_container)),
            1,
        );

        let milk_container = ProviderContainer::new(10, 0.2, String::from("milk container"));
        let foam_container = RechargeableContainer::new(
            5,
            String::from("foam container"),
            0.2,
            ContainerRechargerController::new(Arc::new(milk_container)),
            1,
        );

        let cocoa_container = UnrechargeableContainer::new(5, 0.2, String::from("container"));
        let water_container =
            NetworkRechargeableContainer::new(5, 0.2, 2, String::from("container"));
        let _ = water_container.extract(5);

        let dispenser = Dispenser::new();
        let dispenser_result = dispenser.prepare_order(
            Order {
                order_number: 1,
                coffee_amount,
                cocoa_amount,
                milk_foam_amount,
                water_amount,
            },
            &coffee_container,
            &foam_container,
            &water_container,
            &cocoa_container,
        );
        assert_eq!(dispenser_result, Ok(1));
    }

    #[test]
    fn test13_when_preparing_an_order_and_all_ingredients_except_from_coffee_which_can_be_recharged_are_available_the_dispenser_result_is_one(
    ) {
        let coffee_amount = 3;
        let cocoa_amount = 3;
        let milk_foam_amount = 3;
        let water_amount = 3;

        let grain_container = ProviderContainer::new(10, 0.2, String::from("grain container"));
        let coffee_container = RechargeableContainer::new(
            5,
            String::from("coffee container"),
            0.2,
            ContainerRechargerController::new(Arc::new(grain_container)),
            1,
        );
        let _ = coffee_container.extract(4);

        let milk_container = ProviderContainer::new(10, 0.2, String::from("milk container"));
        let foam_container = RechargeableContainer::new(
            5,
            String::from("foam container"),
            0.2,
            ContainerRechargerController::new(Arc::new(milk_container)),
            1,
        );

        let cocoa_container = UnrechargeableContainer::new(5, 0.2, String::from("container"));
        let water_container =
            NetworkRechargeableContainer::new(5, 0.2, 2, String::from("container"));
        let _ = water_container.extract(5);

        let dispenser = Dispenser::new();
        let dispenser_result = dispenser.prepare_order(
            Order {
                order_number: 1,
                coffee_amount,
                cocoa_amount,
                milk_foam_amount,
                water_amount,
            },
            &coffee_container,
            &foam_container,
            &water_container,
            &cocoa_container,
        );
        assert_eq!(dispenser_result, Ok(1));
    }

    #[test]
    fn test14_when_preparing_an_order_and_all_ingredients_except_from_foam_which_can_be_recharged_are_available_the_dispenser_result_is_one(
    ) {
        let coffee_amount = 3;
        let cocoa_amount = 3;
        let milk_foam_amount = 3;
        let water_amount = 3;

        let grain_container = ProviderContainer::new(10, 0.2, String::from("grain container"));
        let coffee_container = RechargeableContainer::new(
            5,
            String::from("coffee container"),
            0.2,
            ContainerRechargerController::new(Arc::new(grain_container)),
            1,
        );

        let milk_container = ProviderContainer::new(10, 0.2, String::from("milk container"));
        let foam_container = RechargeableContainer::new(
            5,
            String::from("foam container"),
            0.2,
            ContainerRechargerController::new(Arc::new(milk_container)),
            1,
        );
        let _ = foam_container.extract(4);

        let cocoa_container = UnrechargeableContainer::new(5, 0.2, String::from("container"));
        let water_container =
            NetworkRechargeableContainer::new(5, 0.2, 2, String::from("container"));
        let _ = water_container.extract(5);

        let dispenser = Dispenser::new();
        let dispenser_result = dispenser.prepare_order(
            Order {
                order_number: 1,
                coffee_amount,
                cocoa_amount,
                milk_foam_amount,
                water_amount,
            },
            &coffee_container,
            &foam_container,
            &water_container,
            &cocoa_container,
        );
        assert_eq!(dispenser_result, Ok(1));
    }

    #[test]
    fn test15_when_preparing_an_order_with_cero_coffee_and_ingredients_available_the_result_is_one_order_prepared(
    ) {
        let cocoa_amount = 3;
        let milk_foam_amount = 3;
        let water_amount = 3;

        let grain_container = ProviderContainer::new(10, 0.2, String::from("grain container"));
        let coffee_container = RechargeableContainer::new(
            5,
            String::from("coffee container"),
            0.2,
            ContainerRechargerController::new(Arc::new(grain_container)),
            1,
        );

        let milk_container = ProviderContainer::new(10, 0.2, String::from("milk container"));
        let foam_container = RechargeableContainer::new(
            5,
            String::from("foam container"),
            0.2,
            ContainerRechargerController::new(Arc::new(milk_container)),
            1,
        );

        let cocoa_container = UnrechargeableContainer::new(5, 0.2, String::from("container"));
        let water_container =
            NetworkRechargeableContainer::new(5, 0.2, 2, String::from("container"));
        let _ = water_container.extract(5);

        let dispenser = Dispenser::new();
        let dispenser_result = dispenser.prepare_order(
            Order {
                order_number: 1,
                coffee_amount: 0,
                cocoa_amount,
                milk_foam_amount,
                water_amount,
            },
            &coffee_container,
            &foam_container,
            &water_container,
            &cocoa_container,
        );

        assert_eq!(dispenser_result, Ok(1));
    }

    #[test]
    fn test16_when_preparing_an_order_with_cero_cocoa_and_ingredients_available_the_result_is_one_order_prepared(
    ) {
        let coffee_amount = 3;
        let milk_foam_amount = 3;
        let water_amount = 3;

        let grain_container = ProviderContainer::new(10, 0.2, String::from("grain container"));
        let coffee_container = RechargeableContainer::new(
            5,
            String::from("coffee container"),
            0.2,
            ContainerRechargerController::new(Arc::new(grain_container)),
            1,
        );

        let milk_container = ProviderContainer::new(10, 0.2, String::from("milk container"));
        let foam_container = RechargeableContainer::new(
            5,
            String::from("foam container"),
            0.2,
            ContainerRechargerController::new(Arc::new(milk_container)),
            1,
        );

        let cocoa_container = UnrechargeableContainer::new(5, 0.2, String::from("container"));
        let water_container =
            NetworkRechargeableContainer::new(5, 0.2, 2, String::from("container"));
        let _ = water_container.extract(5);

        let dispenser = Dispenser::new();
        let dispenser_result = dispenser.prepare_order(
            Order {
                order_number: 1,
                coffee_amount,
                cocoa_amount: 0,
                milk_foam_amount,
                water_amount,
            },
            &coffee_container,
            &foam_container,
            &water_container,
            &cocoa_container,
        );

        assert_eq!(dispenser_result, Ok(1));
    }

    #[test]
    fn test17_when_preparing_an_order_with_cero_milk_foam_and_ingredients_available_the_result_is_one_order_prepared(
    ) {
        let coffee_amount = 3;
        let cocoa_amount = 3;
        let water_amount = 3;

        let grain_container = ProviderContainer::new(10, 0.2, String::from("grain container"));
        let coffee_container = RechargeableContainer::new(
            5,
            String::from("coffee container"),
            0.2,
            ContainerRechargerController::new(Arc::new(grain_container)),
            1,
        );

        let milk_container = ProviderContainer::new(10, 0.2, String::from("milk container"));
        let foam_container = RechargeableContainer::new(
            5,
            String::from("foam container"),
            0.2,
            ContainerRechargerController::new(Arc::new(milk_container)),
            1,
        );

        let cocoa_container = UnrechargeableContainer::new(5, 0.2, String::from("container"));
        let water_container =
            NetworkRechargeableContainer::new(5, 0.2, 2, String::from("container"));
        let _ = water_container.extract(5);

        let dispenser = Dispenser::new();
        let dispenser_result = dispenser.prepare_order(
            Order {
                order_number: 1,
                coffee_amount,
                cocoa_amount,
                milk_foam_amount: 0,
                water_amount,
            },
            &coffee_container,
            &foam_container,
            &water_container,
            &cocoa_container,
        );

        assert_eq!(dispenser_result, Ok(1));
    }

    #[test]
    fn test17_when_preparing_an_order_with_cero_water_and_ingredients_available_the_result_is_one_order_prepared(
    ) {
        let coffee_amount = 3;
        let cocoa_amount = 3;
        let milk_foam_amount = 3;

        let grain_container = ProviderContainer::new(10, 0.2, String::from("grain container"));
        let coffee_container = RechargeableContainer::new(
            5,
            String::from("coffee container"),
            0.2,
            ContainerRechargerController::new(Arc::new(grain_container)),
            1,
        );

        let milk_container = ProviderContainer::new(10, 0.2, String::from("milk container"));
        let foam_container = RechargeableContainer::new(
            5,
            String::from("foam container"),
            0.2,
            ContainerRechargerController::new(Arc::new(milk_container)),
            1,
        );

        let cocoa_container = UnrechargeableContainer::new(5, 0.2, String::from("container"));
        let water_container =
            NetworkRechargeableContainer::new(5, 0.2, 2, String::from("container"));
        let _ = water_container.extract(5);

        let dispenser = Dispenser::new();
        let dispenser_result = dispenser.prepare_order(
            Order {
                order_number: 1,
                coffee_amount,
                cocoa_amount,
                milk_foam_amount,
                water_amount: 0,
            },
            &coffee_container,
            &foam_container,
            &water_container,
            &cocoa_container,
        );

        assert_eq!(dispenser_result, Ok(1));
    }
}
