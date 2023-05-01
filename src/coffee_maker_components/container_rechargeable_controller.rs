use std::sync::Arc;

use super::{container::Container, provider_container::ProviderContainer};

pub struct ContainerRechargerController {
    recharger_container: Arc<ProviderContainer>,
}

impl ContainerRechargerController {
    pub fn new(recharger_container: Arc<ProviderContainer>) -> Self {
        Self {
            recharger_container,
        }
    }

    pub fn recharge(&self, amount_to_extract: u32) -> Result<u32, String> {
        self.recharger_container.extract(amount_to_extract)
    }
}


#[cfg(test)]
mod test_container_recharger_controller {
    use std::sync::Arc;
    use crate::coffee_maker_components::{provider_container::ProviderContainer, container::Container};
    use super::ContainerRechargerController;

    #[test]
    fn test01_when_needing_to_recharge_three_units_that_are_available_the_extraction_result_is_three() {

        let controller = ContainerRechargerController::new(Arc::new(ProviderContainer::new(10, String::from("Provider"))));
        let result = controller.recharge(3);

        assert_eq!(result, Ok(3));
    }

    #[test]
    fn test02_when_needing_to_recharge_three_units_and_two_are_available_the_extraction_result_is_two() {

        let container = ProviderContainer::new(10, String::from("Provider"));
        let _ = container.extract(8);
        let controller = ContainerRechargerController::new(Arc::new(container));
        let result = controller.recharge(3);

        assert_eq!(result, Ok(2));
    }

    #[test]
    fn test03_when_needing_to_recharge_three_units_and_cero_are_available_the_extraction_result_is_cero() {

        let container = ProviderContainer::new(10, String::from("Provider"));
        let _ = container.extract(10);
        let controller = ContainerRechargerController::new(Arc::new(container));
        let result = controller.recharge(3);

        assert_eq!(result, Ok(0));
    }
}
