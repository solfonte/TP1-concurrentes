use std::{
    sync::{Arc, Condvar, Mutex},
    thread,
    time::Duration,
};

use crate::statistics_checker::statistic::Statatistic;

use super::container::Container;
const NETWORK_LOADING_RATE: u64 = 2; // 2 units per second

#[derive(Debug)]
pub struct System {
    max_capacity: u32,
    amount: u32,
    amount_consumed: u32,
    busy: bool,
}

pub struct NetworkRechargeableContainer {
    /* cantidad actual, is_on, is_busy */
    pair: Arc<(Mutex<System>, Condvar)>,
    name: String,
}

impl Container for NetworkRechargeableContainer {
    fn extract(&self, extraction: u32) -> Result<u32, &str> {
        let mut result = Ok(extraction);
        if let Ok(guard) = self.pair.0.lock() {
            if let Ok(mut system) = self.pair.1.wait_while(guard, |state| state.busy) {
                system.busy = true;
                if system.max_capacity >= extraction && system.amount < extraction {
                    system.amount = self.recharge_from_network(
                        system.max_capacity,
                        system.max_capacity - system.amount,
                    );
                }

                if system.amount >= extraction {
                    system.amount -= extraction;
                    system.amount_consumed += extraction;
                    result = Ok(extraction);
                } else {
                    result = Ok(0);
                }

                system.busy = false;
            }
        }
        self.pair.1.notify_all();
        result
    }

    fn get_statistics(&self) -> Statatistic {
        let mut amount_left = 0;
        let mut amount_consumed = 0;
        if let Ok(guard) = self.pair.0.lock() {
            if let Ok(mut system) = self.pair.1.wait_while(guard, |state| state.busy) {
                system.busy = true;
                amount_left = system.amount;
                amount_consumed = system.amount_consumed;
                system.busy = false;
            }
        }
        //TODO: change name to enum
        Statatistic {
            amount_left,
            amount_consumed,
            container: String::from(&self.name),
        }
    }
}

impl NetworkRechargeableContainer {
    pub fn new(max_capacity: u32, name: String) -> Self {
        Self {
            pair: Arc::new((
                Mutex::new(System {
                    max_capacity,
                    amount: max_capacity,
                    amount_consumed: 0,
                    busy: false,
                }),
                Condvar::new(),
            )),
            name,
        }
    }

    fn recharge_from_network(&self, max_capacity: u32, amount_to_recharge: u32) -> u32 {
        thread::sleep(Duration::from_secs(
            (amount_to_recharge as u64) / NETWORK_LOADING_RATE,
        ));
        max_capacity
    }
}



#[cfg(test)]
mod network_rechargeable_container_test {

    use crate::coffee_maker_components::container::Container;
    use crate::coffee_maker_components::network_rechargeable_container::NetworkRechargeableContainer;
    
    #[test]
    fn test01_when_there_are_two_units_available_then_extracting_cero_is_possible_the_extraction_equals_cero(
    ) {
        let container = NetworkRechargeableContainer::new(2, String::from("Container"));
        let extraction = container.extract(0);

        assert_eq!(extraction, Ok(0))
    }

    #[test]
    fn test02_when_there_are_two_units_available_then_extracting_one_is_possible_the_extraction_equals_one(
    ) {
        let container = NetworkRechargeableContainer::new(2, String::from("Container"));
        let extraction = container.extract(1);

        assert_eq!(extraction, Ok(1))
    }

    #[test]
    fn test03_when_there_are_two_units_available_then_extracting_two_is_possible_the_extraction_equals_two(
    ) {
        let container = NetworkRechargeableContainer::new(2, String::from("Container"));
        let extraction = container.extract(2);

        assert_eq!(extraction, Ok(2))
    }

    #[test]
    fn test04_when_there_are_two_units_available_then_extracting_three_is_not_possible_the_extraction_equals_cero(
    ) {
        let container = NetworkRechargeableContainer::new(2, String::from("Container"));
        let extraction = container.extract(3);

        assert_eq!(extraction, Ok(0))
    }

    #[test]
    fn test05_when_there_are_two_units_available_then_extracting_cero_leaves_two_units_left_available(
    ) {
        let container = NetworkRechargeableContainer::new(2, String::from("Container"));
        let _ = container.extract(0);
        let statistic = container.get_statistics();
        assert_eq!(statistic.amount_left, 2)
    }

    #[test]
    fn test06_when_there_are_two_units_available_then_extracting_one_leaves_one_unit_left_available(
    ) {
        let container = NetworkRechargeableContainer::new(2, String::from("Container"));
        let _ = container.extract(1);
        let statistic = container.get_statistics();
        assert_eq!(statistic.amount_left, 1)
    }

    #[test]
    fn test07_when_there_are_two_units_available_then_extracting_two_leaves_cero_units_left_available(
    ) {
        let container = NetworkRechargeableContainer::new(2, String::from("Container"));
        let _ = container.extract(2);
        let statistic = container.get_statistics();
        assert_eq!(statistic.amount_left, 0)
    }

    #[test]
    fn test08_when_there_are_two_units_available_and_max_capacity_is_two_then_extracting_three_is_not_possible_extraction_equals_two(
    ) {
        let container = NetworkRechargeableContainer::new(2, String::from("Container"));
        let extraction = container.extract(3);
        assert_eq!(extraction, Ok(0));
    }

    #[test]
    fn test09_when_there_are_two_units_available_and_max_capacity_is_two_then_extracting_three_leaves_an_amount_of_two_units_left_available(
    ) {
        let container = NetworkRechargeableContainer::new(2, String::from("Container"));
        let _ = container.extract(3);
        let statistic = container.get_statistics();
        assert_eq!(statistic.amount_left, 2);
    }

    #[test]
    fn test10_when_the_container_is_created_with_max_capacity_of_two_units_the_left_amount_is_two()
    {
        let container = NetworkRechargeableContainer::new(2, String::from("Provider"));
        let statistic = container.get_statistics();
        assert_eq!(statistic.amount_left, 2);
    }
    #[test]
    fn test11_when_there_are_two_units_available_and_max_capacity_is_five_then_extracting_three_is_possible_extraction_equals_three(
    ) {
        let container = NetworkRechargeableContainer::new(5, String::from("Container"));
        let _ = container.extract(3);
        let extraction = container.extract(3);
        let statistic = container.get_statistics();

        assert_eq!(extraction, Ok(3));
        assert_eq!(statistic.amount_left, 2);
    }
}
