mod dispenser_test {
    use crate::coffee_maker::container::Container;

    #[test]
    fn test01_when_dispensing_three_units_from_unrechargeable_container_which_are_available_the_dispenser_returns_three_units(
    ) {
        use crate::coffee_maker::dispenser::Dispenser;
        use crate::coffee_maker::unrechargeable_container::UnrechargeableContainer;

        let units = 3;

        let container = UnrechargeableContainer::new(10, String::from("container"));
        let dispenser = Dispenser::new(0);

        let dispenser_result = dispenser.dispense_resource(units, &container);

        assert_eq!(dispenser_result, Ok(units));
    }

    #[test]
    fn test02_when_dispensing_three_units_from_rechargeable_container_which_are_available_the_dispenser_returns_three_units(
    ) {
        use crate::coffee_maker::dispenser::Dispenser;
        use crate::coffee_maker::rechargeable_container::RechargeableContainer;
        use crate::coffee_maker::{
            container_rechargeable_controller::ContainerRechargerController,
            provider_container::ProviderContainer,
        };
        use std::sync::Arc;

        let units = 3;

        let container = RechargeableContainer::new(
            10,
            String::from("container"),
            ContainerRechargerController::new(Arc::new(ProviderContainer::new(
                0,
                String::from("provider"),
            ))),
            1,
        );
        let dispenser = Dispenser::new(0);

        let dispenser_result = dispenser.dispense_resource(units, &container);

        assert_eq!(dispenser_result, Ok(units));
    }

    #[test]
    fn test03_when_dispensing_three_units_from_network_rechargeable_container_which_are_available_the_dispenser_returns_three_units(
    ) {
        use crate::coffee_maker::dispenser::Dispenser;
        use crate::coffee_maker::network_rechargeable_container::NetworkRechargeableContainer;

        let units = 3;

        let container = NetworkRechargeableContainer::new(10, String::from("container"));
        let dispenser = Dispenser::new(0);

        let dispenser_result = dispenser.dispense_resource(units, &container);

        assert_eq!(dispenser_result, Ok(units));
    }

    #[test]
    fn test04_when_dispensing_three_units_from_unrechargeable_container_which_are_not_available_the_dispenser_returns_cero_units(
    ) {
        use crate::coffee_maker::dispenser::Dispenser;
        use crate::coffee_maker::unrechargeable_container::UnrechargeableContainer;

        let units = 3;

        let container = UnrechargeableContainer::new(5, String::from("container"));
        let dispenser = Dispenser::new(0);
        let _ = container.extract(4);
        let dispenser_result = dispenser.dispense_resource(units, &container);

        assert_eq!(dispenser_result, Ok(0));
    }

    #[test]
    fn test05_when_dispensing_three_units_from_rechargeable_container_which_are_not_available_and_cannot_be_recharged_the_dispenser_returns_cero_units(
    ) {
        use crate::coffee_maker::dispenser::Dispenser;
        use crate::coffee_maker::rechargeable_container::RechargeableContainer;
        use crate::coffee_maker::{
            container_rechargeable_controller::ContainerRechargerController,
            provider_container::ProviderContainer,
        };
        use std::sync::Arc;

        let units = 3;

        let container = RechargeableContainer::new(
            5,
            String::from("container"),
            ContainerRechargerController::new(Arc::new(ProviderContainer::new(
                0,
                String::from("provider"),
            ))),
            1,
        );
        let _ = container.extract(4);
        let dispenser = Dispenser::new(0);
        let dispenser_result = dispenser.dispense_resource(units, &container);
        assert_eq!(dispenser_result, Ok(0));
    }

    #[test]
    fn test06_when_dispensing_three_units_from_network_rechargeable_container_which_are_not_available_but_is_below_max_capacity_the_dispenser_returns_three_units(
    ) {
        use crate::coffee_maker::dispenser::Dispenser;
        use crate::coffee_maker::network_rechargeable_container::NetworkRechargeableContainer;

        let units = 3;

        let container = NetworkRechargeableContainer::new(5, String::from("container"));
        let dispenser = Dispenser::new(0);
        let _ = container.extract(4);
        let dispenser_result = dispenser.dispense_resource(units, &container);

        assert_eq!(dispenser_result, Ok(units));
    }
}
