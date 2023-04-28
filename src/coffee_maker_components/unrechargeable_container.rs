use std::sync::{Arc, Condvar, Mutex};

use crate::statistics_checker::statistic::Statatistic;

use super::container::Container;
#[derive(Debug)]
pub struct System {
    amount: u32,
    amount_consumed: u32,
    busy: bool,
    is_on: bool,
}

pub struct UnrechargeableContainer {
    /* cantidad actual, is_on, is_busy */
    pair: Arc<(Mutex<System>, Condvar)>,
    name: String,
}

impl Container for UnrechargeableContainer {
    fn extract(&self, extraction: u32) -> Result<u32, &str> {
        let mut result: Result<u32, &str> = Err("No se pudo extraer del contenedor");
        if let Ok(guard) = self.pair.0.lock() {
            if let Ok(mut system) = self
                .pair
                .1
                .wait_while(guard, |state| state.busy && state.is_on)
            {
                system.busy = true;

                if system.amount < extraction {
                    result = Ok(0);
                } else {
                    system.amount -= extraction;
                    system.amount_consumed += extraction;
                    result = Ok(extraction);
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
        Statatistic {
            amount_left,
            amount_consumed,
            container: String::from(&self.name),
        }
    }
}

impl UnrechargeableContainer {
    pub fn new(max_capacity: u32, name: String) -> Self {
        Self {
            pair: Arc::new((
                Mutex::new(System {
                    amount: max_capacity,
                    amount_consumed: 0,
                    busy: false,
                    is_on: true,
                }),
                Condvar::new(),
            )),
            name,
        }
    }
}

#[cfg(test)]
//TODO: arreglar el tema de los use crate:: .....
mod unrechargeable_container_test {
    use crate::coffee_maker_components::container::Container;
    use crate::coffee_maker_components::unrechargeable_container::UnrechargeableContainer;

    #[test]
    fn test01_when_there_are_two_units_available_then_extracting_cero_is_possible_the_extraction_equals_cero(
    ) {
        let container = UnrechargeableContainer::new(2, String::from("unit container"));
        let extraction = container.extract(0);

        assert_eq!(extraction, Ok(0))
    }

    #[test]
    fn test02_when_there_are_two_units_available_then_extracting_one_is_possible_the_extraction_equals_one(
    ) {
        let container = UnrechargeableContainer::new(2, String::from("unit container"));
        let extraction = container.extract(1);

        assert_eq!(extraction, Ok(1))
    }

    #[test]
    fn test03_when_there_are_two_units_available_then_extracting_two_is_possible_the_extraction_equals_two(
    ) {
        let container = UnrechargeableContainer::new(2, String::from("unit container"));
        let extraction = container.extract(2);

        assert_eq!(extraction, Ok(2))
    }

    #[test]
    fn test04_when_there_are_two_units_available_then_extracting_three_is_not_possible_the_extraction_equals_cero(
    ) {
        let container = UnrechargeableContainer::new(2, String::from("unit container"));
        let extraction = container.extract(3);

        assert_eq!(extraction, Ok(0))
    }
    #[test]
    fn test05_when_there_are_two_units_available_then_extracting_cero_leaves_an_amount_of_two_units_left_available(
    ) {
        let container = UnrechargeableContainer::new(2, String::from("unit container"));
        let _ = container.extract(0);
        let statistic = container.get_statistics();
        assert_eq!(statistic.amount_left, 2)
    }

    #[test]
    fn test06_when_there_are_two_units_available_then_extracting_one_leaves_an_amount_of_one_unit_left_available(
    ) {
        let container = UnrechargeableContainer::new(2, String::from("unit container"));
        let _ = container.extract(1);
        let statistic = container.get_statistics();
        assert_eq!(statistic.amount_left, 1)
    }

    #[test]
    fn test07_when_there_are_two_units_available_then_extracting_two_leaves_an_amount_of_cero_units_left_available(
    ) {
        let container = UnrechargeableContainer::new(2, String::from("unit container"));
        let _ = container.extract(2);
        let statistic = container.get_statistics();
        assert_eq!(statistic.amount_left, 0);
    }

    #[test]
    fn test08_when_there_are_two_units_available_then_extracting_three_leaves_an_amount_of_two_units_left_available(
    ) {
        let container = UnrechargeableContainer::new(2, String::from("unit container"));
        let _ = container.extract(3);
        let statistic = container.get_statistics();
        assert_eq!(statistic.amount_left, 2)
    }

    #[test]
    fn test09_when_the_container_is_created_with_max_capacity_of_two_units_the_left_amount_is_two()
    {
        let container = UnrechargeableContainer::new(2, String::from("unit container"));
        let statistic = container.get_statistics();
        assert_eq!(statistic.amount_left, 2)
    }
}
