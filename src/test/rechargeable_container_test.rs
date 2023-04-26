mod rechargeable_container_test {
    #[test]
    fn test01_when_there_are_two_units_available_then_extracting_cero_is_possible_the_extraction_equals_cero(
    ) {
        use crate::coffee_maker::container::Container;
        use crate::coffee_maker::container_rechargeable_controller::ContainerRechargerController;
        use crate::coffee_maker::provider_container::ProviderContainer;
        use crate::coffee_maker::rechargeable_container::RechargeableContainer;
        use std::sync::Arc;

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
        use crate::coffee_maker::container::Container;
        use crate::coffee_maker::container_rechargeable_controller::ContainerRechargerController;
        use crate::coffee_maker::provider_container::ProviderContainer;
        use crate::coffee_maker::rechargeable_container::RechargeableContainer;
        use std::sync::Arc;

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
        use crate::coffee_maker::container::Container;
        use crate::coffee_maker::container_rechargeable_controller::ContainerRechargerController;
        use crate::coffee_maker::provider_container::ProviderContainer;
        use crate::coffee_maker::rechargeable_container::RechargeableContainer;
        use std::sync::Arc;

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
        use crate::coffee_maker::container::Container;
        use crate::coffee_maker::container_rechargeable_controller::ContainerRechargerController;
        use crate::coffee_maker::provider_container::ProviderContainer;
        use crate::coffee_maker::rechargeable_container::RechargeableContainer;
        use std::sync::Arc;

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
        use crate::coffee_maker::container::Container;
        use crate::coffee_maker::container_rechargeable_controller::ContainerRechargerController;
        use crate::coffee_maker::provider_container::ProviderContainer;
        use crate::coffee_maker::rechargeable_container::RechargeableContainer;
        use std::sync::Arc;

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
        use crate::coffee_maker::container::Container;
        use crate::coffee_maker::container_rechargeable_controller::ContainerRechargerController;
        use crate::coffee_maker::provider_container::ProviderContainer;
        use crate::coffee_maker::rechargeable_container::RechargeableContainer;
        use std::sync::Arc;

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
        use crate::coffee_maker::container::Container;
        use crate::coffee_maker::container_rechargeable_controller::ContainerRechargerController;
        use crate::coffee_maker::provider_container::ProviderContainer;
        use crate::coffee_maker::rechargeable_container::RechargeableContainer;
        use std::sync::Arc;

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
        use crate::coffee_maker::container::Container;
        use crate::coffee_maker::container_rechargeable_controller::ContainerRechargerController;
        use crate::coffee_maker::provider_container::ProviderContainer;
        use crate::coffee_maker::rechargeable_container::RechargeableContainer;
        use std::sync::Arc;

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
        use crate::coffee_maker::container::Container;
        use crate::coffee_maker::container_rechargeable_controller::ContainerRechargerController;
        use crate::coffee_maker::provider_container::ProviderContainer;
        use crate::coffee_maker::rechargeable_container::RechargeableContainer;
        use std::sync::Arc;

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
        use crate::coffee_maker::container::Container;
        use crate::coffee_maker::container_rechargeable_controller::ContainerRechargerController;
        use crate::coffee_maker::provider_container::ProviderContainer;
        use crate::coffee_maker::rechargeable_container::RechargeableContainer;
        use std::sync::Arc;

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
        use crate::coffee_maker::container::Container;
        use crate::coffee_maker::container_rechargeable_controller::ContainerRechargerController;
        use crate::coffee_maker::provider_container::ProviderContainer;
        use crate::coffee_maker::rechargeable_container::RechargeableContainer;
        use std::sync::Arc;

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
        use crate::coffee_maker::container::Container;
        use crate::coffee_maker::container_rechargeable_controller::ContainerRechargerController;
        use crate::coffee_maker::provider_container::ProviderContainer;
        use crate::coffee_maker::rechargeable_container::RechargeableContainer;
        use std::sync::Arc;

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
    //TODO: tests de los casos de recarga
}
