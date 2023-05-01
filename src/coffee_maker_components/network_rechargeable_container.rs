use std::{
    sync::{Arc, Condvar, Mutex},
    thread,
    time::Duration,
};

use crate::statistics_checker::statistic::Statistic;

use super::{container::Container, container_system::ContainerSystem};
const NETWORK_LOADING_RATE: u64 = 2; // 2 units per second

pub struct NetworkRechargeableContainer {
    max_capacity: u32,
    amount_percentage_alert: f32,
    pair: Arc<(Mutex<ContainerSystem>, Condvar)>,
    name: String,
}

impl Container for NetworkRechargeableContainer {
    fn extract(&self, extraction: u32) -> Result<u32, String> {
        let mut result = Ok(extraction);
        if let Ok(guard) = self.pair.0.lock() {
            if let Ok(mut system) = self.pair.1.wait_while(guard, |state| state.is_busy()) {
                result = self.extract_amount(&mut system, extraction);
            }
        }
        self.pair.1.notify_all();
        result
    }

    fn get_statistics(&self) -> Statistic {
        let mut amount_left = 0;
        let mut amount_consumed = 0;
        if let Ok(guard) = self.pair.0.lock() {
            if let Ok(mut system) = self.pair.1.wait_while(guard, |state| state.is_busy()) {
                system.set_busy(true);
                amount_left = system.get_amount_left();
                amount_consumed = system.get_amount_consumed();
                system.set_busy(false);
            }
        }

        Statistic {
            amount_left,
            amount_consumed,
            container: String::from(&self.name),
        }
    }
}

impl NetworkRechargeableContainer {
    pub fn new(max_capacity: u32, amount_percentage_alert: f32, name: String) -> Self {
        Self {
            pair: Arc::new((
                Mutex::new(ContainerSystem::new(max_capacity)),
                Condvar::new(),
            )),
            amount_percentage_alert,
            max_capacity,
            name,
        }
    }

    fn recharge_from_network(&self, max_capacity: u32, amount_to_recharge: u32) -> u32 {
        thread::sleep(Duration::from_millis(
            (amount_to_recharge as u64) / NETWORK_LOADING_RATE,
        ));
        max_capacity
    }

    pub fn extract_amount(
        &self,
        system: &mut ContainerSystem,
        extraction: u32,
    ) -> Result<u32, String> {
        system.set_busy(true);
        let amount_left = system.get_amount_left();
        if self.max_capacity >= extraction && amount_left < extraction {
            self.recharge_from_network(self.max_capacity, self.max_capacity - amount_left);

            system.recharge(self.max_capacity - amount_left);
            system.set_already_alerted_amount_percentage(false);

        }

        let result: Result<u32, String> = if system.get_amount_left() >= extraction {
            system.extract(extraction);
            system.increase_amount_consumed(extraction);
            Ok(extraction)
        } else {
            Ok(0)
        };

        if !system.already_alerted_amount_percentage() && self.check_alert_on_amount_left_percentage(
            &self.name,
            system.get_amount_left(),
            self.max_capacity,
            self.amount_percentage_alert,
        ) {
            system.set_already_alerted_amount_percentage(true);
        }

        system.set_busy(false);
        result
    }
}

#[cfg(test)]
mod network_rechargeable_container_test {

    use crate::coffee_maker_components::container::Container;
    use crate::coffee_maker_components::network_rechargeable_container::NetworkRechargeableContainer;

    #[test]
    fn test01_when_there_are_two_units_available_then_extracting_cero_is_possible_the_extraction_equals_cero(
    ) {
        let container = NetworkRechargeableContainer::new(2, 0.2, String::from("Container"));
        let extraction = container.extract(0);

        assert_eq!(extraction, Ok(0))
    }

    #[test]
    fn test02_when_there_are_two_units_available_then_extracting_one_is_possible_the_extraction_equals_one(
    ) {
        let container = NetworkRechargeableContainer::new(2, 0.2, String::from("Container"));
        let extraction = container.extract(1);

        assert_eq!(extraction, Ok(1))
    }

    #[test]
    fn test03_when_there_are_two_units_available_then_extracting_two_is_possible_the_extraction_equals_two(
    ) {
        let container = NetworkRechargeableContainer::new(2, 0.2, String::from("Container"));
        let extraction = container.extract(2);

        assert_eq!(extraction, Ok(2))
    }

    #[test]
    fn test04_when_there_are_two_units_available_then_extracting_three_is_not_possible_the_extraction_equals_cero(
    ) {
        let container = NetworkRechargeableContainer::new(2, 0.2, String::from("Container"));
        let extraction = container.extract(3);

        assert_eq!(extraction, Ok(0))
    }

    #[test]
    fn test05_when_there_are_two_units_available_then_extracting_cero_leaves_two_units_left_available(
    ) {
        let container = NetworkRechargeableContainer::new(2, 0.2, String::from("Container"));
        let _ = container.extract(0);
        let statistic = container.get_statistics();
        assert_eq!(statistic.amount_left, 2)
    }

    #[test]
    fn test06_when_there_are_two_units_available_then_extracting_one_leaves_one_unit_left_available(
    ) {
        let container = NetworkRechargeableContainer::new(2, 0.2, String::from("Container"));
        let _ = container.extract(1);
        let statistic = container.get_statistics();
        assert_eq!(statistic.amount_left, 1)
    }

    #[test]
    fn test07_when_there_are_two_units_available_then_extracting_two_leaves_cero_units_left_available(
    ) {
        let container = NetworkRechargeableContainer::new(2, 0.2, String::from("Container"));
        let _ = container.extract(2);
        let statistic = container.get_statistics();
        assert_eq!(statistic.amount_left, 0)
    }

    #[test]
    fn test08_when_there_are_two_units_available_and_max_capacity_is_two_then_extracting_three_is_not_possible_extraction_equals_two(
    ) {
        let container = NetworkRechargeableContainer::new(2, 0.2, String::from("Container"));
        let extraction = container.extract(3);
        assert_eq!(extraction, Ok(0));
    }

    #[test]
    fn test09_when_there_are_two_units_available_and_max_capacity_is_two_then_extracting_three_leaves_an_amount_of_two_units_left_available(
    ) {
        let container = NetworkRechargeableContainer::new(2, 0.2, String::from("Container"));
        let _ = container.extract(3);
        let statistic = container.get_statistics();
        assert_eq!(statistic.amount_left, 2);
    }

    #[test]
    fn test10_when_the_container_is_created_with_max_capacity_of_two_units_the_left_amount_is_two()
    {
        let container = NetworkRechargeableContainer::new(2, 0.2, String::from("Provider"));
        let statistic = container.get_statistics();
        assert_eq!(statistic.amount_left, 2);
    }
    #[test]
    fn test11_when_there_are_two_units_available_and_max_capacity_is_five_then_extracting_three_is_possible_extraction_equals_three(
    ) {
        let container = NetworkRechargeableContainer::new(5, 0.2, String::from("Container"));
        let _ = container.extract(3);
        let extraction = container.extract(3);
        let statistic = container.get_statistics();

        assert_eq!(extraction, Ok(3));
        assert_eq!(statistic.amount_left, 2);
    }
}
