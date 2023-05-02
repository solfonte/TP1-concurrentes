use std::sync::{Arc, Condvar, Mutex};

use crate::statistics_checker::statistic::Statistic;

use super::{container::Container, container_system::ContainerSystem};
pub struct UnrechargeableContainer {
    pair: Arc<(Mutex<ContainerSystem>, Condvar)>,
    amount_percentage_alert: f32,
    max_capacity: u32,
    name: String,
}

impl Container for UnrechargeableContainer {
    fn extract(&self, extraction: u32) -> Result<u32, String> {
        let mut result: Result<u32, String> =
            Err(String::from("No se pudo extraer del contenedor"));
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

impl UnrechargeableContainer {
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

    pub fn extract_amount(
        &self,
        system: &mut ContainerSystem,
        extraction: u32,
    ) -> Result<u32, String> {
        system.set_busy(true);

        let amount_left = system.get_amount_left();
        let result = if amount_left < extraction {
            Ok(0)
        } else {
            system.extract(extraction);
            system.increase_amount_consumed(extraction);
            Ok(extraction)
        };

        if !system.already_alerted_amount_percentage()
            && self.check_alert_on_amount_left_percentage(
                &self.name,
                system.get_amount_consumed(),
                self.max_capacity,
                self.amount_percentage_alert,
            )
        {
            system.set_already_alerted_amount_percentage(true);
        }

        system.set_busy(false);

        result
    }
}

#[cfg(test)]
mod unrechargeable_container_test {
    use crate::coffee_maker_components::container::Container;
    use crate::coffee_maker_components::unrechargeable_container::UnrechargeableContainer;

    #[test]
    fn test01_when_there_are_two_units_available_then_extracting_cero_is_possible_the_extraction_equals_cero(
    ) {
        let container = UnrechargeableContainer::new(2, 0.2, String::from("unit container"));
        let extraction = container.extract(0);

        assert_eq!(extraction, Ok(0))
    }

    #[test]
    fn test02_when_there_are_two_units_available_then_extracting_one_is_possible_the_extraction_equals_one(
    ) {
        let container = UnrechargeableContainer::new(2, 0.2, String::from("unit container"));
        let extraction = container.extract(1);

        assert_eq!(extraction, Ok(1))
    }

    #[test]
    fn test03_when_there_are_two_units_available_then_extracting_two_is_possible_the_extraction_equals_two(
    ) {
        let container = UnrechargeableContainer::new(2, 0.2, String::from("unit container"));
        let extraction = container.extract(2);

        assert_eq!(extraction, Ok(2))
    }

    #[test]
    fn test04_when_there_are_two_units_available_then_extracting_three_is_not_possible_the_extraction_equals_cero(
    ) {
        let container = UnrechargeableContainer::new(2, 0.2, String::from("unit container"));
        let extraction = container.extract(3);

        assert_eq!(extraction, Ok(0))
    }
    #[test]
    fn test05_when_there_are_two_units_available_then_extracting_cero_leaves_an_amount_of_two_units_left_available(
    ) {
        let container = UnrechargeableContainer::new(2, 0.2, String::from("unit container"));
        let _ = container.extract(0);
        let statistic = container.get_statistics();
        assert_eq!(statistic.amount_left, 2)
    }

    #[test]
    fn test06_when_there_are_two_units_available_then_extracting_one_leaves_an_amount_of_one_unit_left_available(
    ) {
        let container = UnrechargeableContainer::new(2, 0.2, String::from("unit container"));
        let _ = container.extract(1);
        let statistic = container.get_statistics();
        assert_eq!(statistic.amount_left, 1)
    }

    #[test]
    fn test07_when_there_are_two_units_available_then_extracting_two_leaves_an_amount_of_cero_units_left_available(
    ) {
        let container = UnrechargeableContainer::new(2, 0.2, String::from("unit container"));
        let _ = container.extract(2);
        let statistic = container.get_statistics();
        assert_eq!(statistic.amount_left, 0);
    }

    #[test]
    fn test08_when_there_are_two_units_available_then_extracting_three_leaves_an_amount_of_two_units_left_available(
    ) {
        let container = UnrechargeableContainer::new(2, 0.2, String::from("unit container"));
        let _ = container.extract(3);
        let statistic = container.get_statistics();
        assert_eq!(statistic.amount_left, 2)
    }

    #[test]
    fn test09_when_the_container_is_created_with_max_capacity_of_two_units_the_left_amount_is_two()
    {
        let container = UnrechargeableContainer::new(2, 0.2, String::from("unit container"));
        let statistic = container.get_statistics();
        assert_eq!(statistic.amount_left, 2)
    }
}
