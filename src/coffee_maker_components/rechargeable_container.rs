use std::sync::{Arc, Condvar, Mutex};

use crate::statistics_checker::statistic::Statistic;

use super::{
    container::Container, container_rechargeable_controller::ContainerRechargerController,
};

#[derive(Debug)]
pub struct System {
    max_capacity: u32,
    amount: u32,
    amount_consumed: u32,
    busy: bool,
    is_on: bool,
}

pub struct RechargeableContainer {
    pair: Arc<(Mutex<System>, Condvar)>,
    name: String,
    recharger_controller: ContainerRechargerController,
    recharging_rate: u32,
}

impl Container for RechargeableContainer {
    fn extract(&self, extraction: u32) -> Result<u32, String> {
        let mut result: Result<u32, String> = Err(String::from("No se pudo extraer del contenedor"));
        if let Ok(guard) = self.pair.0.lock() {
            if let Ok(mut system) = self
                .pair
                .1
                .wait_while(guard, |state| state.busy && state.is_on)
            {
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
            if let Ok(mut system) = self.pair.1.wait_while(guard, |state| state.busy) {
                system.busy = true;
                amount_left = system.amount;
                amount_consumed = system.amount_consumed;
                system.busy = false;
            }
        }
        Statistic {
            amount_left,
            amount_consumed,
            container: String::from(&self.name),
        }
    }
}

impl RechargeableContainer {
    pub fn new(
        max_capacity: u32,
        name: String,
        recharger_controller: ContainerRechargerController,
        recharging_rate: u32,
    ) -> Self {
        Self {
            pair: Arc::new((
                Mutex::new(System {
                    max_capacity,
                    amount: max_capacity,
                    amount_consumed: 0,
                    busy: false,
                    is_on: true,
                }),
                Condvar::new(),
            )),
            name,
            recharger_controller,
            recharging_rate,
        }
    }

    pub fn extract_amount(&self, system: &mut System, extraction: u32) -> Result<u32, String> {
        system.busy = true;

        if system.amount < extraction {
            let amount_to_recharge = (system.max_capacity - system.amount) / self.recharging_rate;
            let recharging_result = self.recharger_controller.recharge(amount_to_recharge);
            if let Ok(amount_returned) = recharging_result {
                println!("[CONTAINER COULD RECHARGE!]");
                system.amount += amount_returned * self.recharging_rate;
            }
        }

        let result = if system.amount >= extraction {
            system.amount -= extraction;
            system.amount_consumed += extraction;
            Ok(extraction)
        } else {
            Ok(0)
        };

        system.busy = false;
        result
    }
}

#[cfg(test)]
mod rechargeable_container_test {
    use crate::coffee_maker_components::container::Container;
    use crate::coffee_maker_components::container_rechargeable_controller::ContainerRechargerController;
    use crate::coffee_maker_components::provider_container::ProviderContainer;
    use crate::coffee_maker_components::rechargeable_container::RechargeableContainer;
    use std::sync::Arc;

    #[test]
    fn test01_when_there_are_two_units_available_then_extracting_cero_is_possible_the_extraction_equals_cero(
    ) {
        let container_recharger_controller = ContainerRechargerController::new(Arc::new(
            ProviderContainer::new(0, String::from("Provider")),
        ));
        let container = RechargeableContainer::new(
            2,
            String::from("Rechargeable container"),
            container_recharger_controller,
            1,
        );
        let extraction = container.extract(0);
        assert_eq!(extraction, Ok(0))
    }

    #[test]
    fn test02_when_there_are_two_units_available_then_extracting_one_is_possible_the_extraction_equals_one(
    ) {
        let container_recharger_controller = ContainerRechargerController::new(Arc::new(
            ProviderContainer::new(0, String::from("Provider")),
        ));
        let container = RechargeableContainer::new(
            2,
            String::from("Rechargeable container"),
            container_recharger_controller,
            1,
        );
        let extraction = container.extract(1);
        assert_eq!(extraction, Ok(1))
    }

    #[test]
    fn test03_when_there_are_two_units_available_then_extracting_two_is_possible_the_extraction_equals_two(
    ) {
        let container_recharger_controller = ContainerRechargerController::new(Arc::new(
            ProviderContainer::new(0, String::from("Provider")),
        ));
        let container = RechargeableContainer::new(
            2,
            String::from("Rechargeable container"),
            container_recharger_controller,
            1,
        );
        let extraction = container.extract(2);
        assert_eq!(extraction, Ok(2))
    }

    #[test]
    fn test04_when_there_are_two_units_available_and_no_recharging_resource_available_then_extracting_three_is_not_possible_the_extraction_equals_cero(
    ) {
        let container_recharger_controller = ContainerRechargerController::new(Arc::new(
            ProviderContainer::new(0, String::from("Provider")),
        ));
        let container = RechargeableContainer::new(
            2,
            String::from("Rechargeable container"),
            container_recharger_controller,
            1,
        );
        let extraction = container.extract(3);
        assert_eq!(extraction, Ok(0))
    }

    #[test]
    fn test05_when_there_are_two_units_available_and_max_capacity_is_two_and_three_recharging_units_available_then_extracting_five_is_not_possible_the_extraction_equals_cero(
    ) {
        let container_recharger_controller = ContainerRechargerController::new(Arc::new(
            ProviderContainer::new(3, String::from("Provider")),
        ));
        let container = RechargeableContainer::new(
            2,
            String::from("Rechargeable container"),
            container_recharger_controller,
            1,
        );
        let extraction = container.extract(5);
        assert_eq!(extraction, Ok(0))
    }

    #[test]
    fn test06_when_there_are_two_units_available_and_max_capacity_is_five_and_three_recharging_units_available_then_extracting_five_is_possible_the_extraction_equals_five(
    ) {
        let container_recharger_controller = ContainerRechargerController::new(Arc::new(
            ProviderContainer::new(3, String::from("Provider")),
        ));
        let container = RechargeableContainer::new(
            5,
            String::from("Rechargeable container"),
            container_recharger_controller,
            1,
        );
        let _ = container.extract(3);
        let extraction = container.extract(5);
        assert_eq!(extraction, Ok(5))
    }

    #[test]
    fn test07_when_there_are_two_units_available_then_extracting_cero_leaves_an_amount_of_two_units_left_available(
    ) {
        let container_recharger_controller = ContainerRechargerController::new(Arc::new(
            ProviderContainer::new(0, String::from("Provider")),
        ));
        let container = RechargeableContainer::new(
            2,
            String::from("Rechargeable container"),
            container_recharger_controller,
            1,
        );
        let _ = container.extract(0);
        let statistic = container.get_statistics();
        assert_eq!(statistic.amount_left, 2);
    }

    #[test]
    fn test08_when_there_are_two_units_available_then_extracting_one_leaves_an_amount_of_one_unit_left_available(
    ) {
        let container_recharger_controller = ContainerRechargerController::new(Arc::new(
            ProviderContainer::new(0, String::from("Provider")),
        ));
        let container = RechargeableContainer::new(
            2,
            String::from("Rechargeable container"),
            container_recharger_controller,
            1,
        );
        let _ = container.extract(1);
        let statistic = container.get_statistics();
        assert_eq!(statistic.amount_left, 1);
    }

    #[test]
    fn test09_when_there_are_two_units_available_then_extracting_two_leaves_an_amount_of_cero_units_left_available(
    ) {
        let container_recharger_controller = ContainerRechargerController::new(Arc::new(
            ProviderContainer::new(0, String::from("Provider")),
        ));
        let container = RechargeableContainer::new(
            2,
            String::from("Rechargeable container"),
            container_recharger_controller,
            1,
        );
        let _ = container.extract(2);
        let statistic = container.get_statistics();
        assert_eq!(statistic.amount_left, 0);
    }

    #[test]
    fn test10_when_there_are_two_units_available_and_no_recharging_resource_available_then_extracting_three_leaves_an_amount_of_two_units_left_available(
    ) {
        let container_recharger_controller = ContainerRechargerController::new(Arc::new(
            ProviderContainer::new(0, String::from("Provider")),
        ));
        let container = RechargeableContainer::new(
            2,
            String::from("Rechargeable container"),
            container_recharger_controller,
            1,
        );
        let _ = container.extract(3);
        let statistic = container.get_statistics();
        assert_eq!(statistic.amount_left, 2);
    }

    #[test]
    fn test11_when_there_are_two_units_available_and_max_capacity_is_two_and_three_recharging_units_available_then_extracting_five_leaves_an_amount_of_two_units_left_available(
    ) {
        let container_recharger_controller = ContainerRechargerController::new(Arc::new(
            ProviderContainer::new(3, String::from("Provider")),
        ));
        let container = RechargeableContainer::new(
            2,
            String::from("Rechargeable container"),
            container_recharger_controller,
            1,
        );
        let _ = container.extract(5);
        let statistic = container.get_statistics();
        assert_eq!(statistic.amount_left, 2);
    }

    #[test]
    fn test12_when_the_container_is_created_with_max_capacity_of_two_units_the_left_amount_is_two()
    {
        let container_recharger_controller = ContainerRechargerController::new(Arc::new(
            ProviderContainer::new(3, String::from("Provider")),
        ));
        let container = RechargeableContainer::new(
            2,
            String::from("Rechargeable container"),
            container_recharger_controller,
            1,
        );
        let statistic = container.get_statistics();
        assert_eq!(statistic.amount_left, 2);
    }
}
