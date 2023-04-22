mod dispenser_test {

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
        use crate::coffee_maker::container::Container;
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
        use crate::coffee_maker::container::Container;
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
        use crate::coffee_maker::container::Container;
        use crate::coffee_maker::dispenser::Dispenser;
        use crate::coffee_maker::network_rechargeable_container::NetworkRechargeableContainer;

        let units = 3;

        let container = NetworkRechargeableContainer::new(5, String::from("container"));
        let dispenser = Dispenser::new(0);
        let _ = container.extract(4);
        let dispenser_result = dispenser.dispense_resource(units, &container);

        assert_eq!(dispenser_result, Ok(units));
    }

    #[test]
    fn test07_when_dispensing_three_units_from_rechargeable_container_which_are_available_and_can_be_recharged_the_dispenser_returns_three_units(
    ) {
        use crate::coffee_maker::container::Container;
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
                5,
                String::from("provider"),
            ))),
            1,
        );
        let _ = container.extract(4);
        let dispenser = Dispenser::new(0);
        let dispenser_result = dispenser.dispense_resource(units, &container);
        assert_eq!(dispenser_result, Ok(3));
    }

    #[test]
    fn test08_when_preparing_an_order_and_all_ingredients_are_available_the_dispenser_result_is_one(
    ) {
        use crate::coffee_maker::dispenser::Dispenser;
        use crate::coffee_maker::network_rechargeable_container::NetworkRechargeableContainer;
        use crate::coffee_maker::rechargeable_container::RechargeableContainer;
        use crate::coffee_maker::{
            container_rechargeable_controller::ContainerRechargerController,
            provider_container::ProviderContainer,
        };
        use crate::{
            coffee_maker::unrechargeable_container::UnrechargeableContainer, order::order::Order,
        };
        use std::sync::Arc;

        let coffee_units = 3;
        let cocoa_units = 3;
        let foam_units = 3;
        let water_units = 3;

        let grain_container = ProviderContainer::new(10, String::from("grain container"));
        let coffee_container = RechargeableContainer::new(
            5,
            String::from("coffee container"),
            ContainerRechargerController::new(Arc::new(grain_container)),
            1,
        );

        let milk_container = ProviderContainer::new(10, String::from("milk container"));
        let foam_container = RechargeableContainer::new(
            5,
            String::from("foam container"),
            ContainerRechargerController::new(Arc::new(milk_container)),
            1,
        );
        let cocoa_container = UnrechargeableContainer::new(5, String::from("container"));
        let water_container = NetworkRechargeableContainer::new(5, String::from("container"));

        let dispenser = Dispenser::new(0);
        let dispenser_result = dispenser.prepare_order(
            Order::new(1, coffee_units, cocoa_units, foam_units, water_units),
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
        use crate::coffee_maker::container::Container;
        use crate::coffee_maker::dispenser::Dispenser;
        use crate::coffee_maker::network_rechargeable_container::NetworkRechargeableContainer;
        use crate::coffee_maker::rechargeable_container::RechargeableContainer;
        use crate::coffee_maker::{
            container_rechargeable_controller::ContainerRechargerController,
            provider_container::ProviderContainer,
        };
        use crate::{
            coffee_maker::unrechargeable_container::UnrechargeableContainer, order::order::Order,
        };
        use std::sync::Arc;

        let coffee_units = 3;
        let cocoa_units = 3;
        let foam_units = 3;
        let water_units = 3;

        let grain_container = ProviderContainer::new(10, String::from("grain container"));
        let coffee_container = RechargeableContainer::new(
            5,
            String::from("coffee container"),
            ContainerRechargerController::new(Arc::new(grain_container)),
            1,
        );

        let milk_container = ProviderContainer::new(10, String::from("milk container"));
        let foam_container = RechargeableContainer::new(
            5,
            String::from("foam container"),
            ContainerRechargerController::new(Arc::new(milk_container)),
            1,
        );
        let cocoa_container = UnrechargeableContainer::new(5, String::from("container"));
        let _ = cocoa_container.extract(5);
        let water_container = NetworkRechargeableContainer::new(5, String::from("container"));

        let dispenser = Dispenser::new(0);
        let dispenser_result = dispenser.prepare_order(
            Order::new(1, coffee_units, cocoa_units, foam_units, water_units),
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
        use crate::coffee_maker::container::Container;
        use crate::coffee_maker::dispenser::Dispenser;
        use crate::coffee_maker::network_rechargeable_container::NetworkRechargeableContainer;
        use crate::coffee_maker::rechargeable_container::RechargeableContainer;
        use crate::coffee_maker::{
            container_rechargeable_controller::ContainerRechargerController,
            provider_container::ProviderContainer,
        };
        use crate::{
            coffee_maker::unrechargeable_container::UnrechargeableContainer, order::order::Order,
        };
        use std::sync::Arc;

        let coffee_units = 3;
        let cocoa_units = 3;
        let foam_units = 3;
        let water_units = 3;

        let grain_container = ProviderContainer::new(0, String::from("grain container"));
        let coffee_container = RechargeableContainer::new(
            5,
            String::from("coffee container"),
            ContainerRechargerController::new(Arc::new(grain_container)),
            1,
        );
        let _ = coffee_container.extract(5);

        let milk_container = ProviderContainer::new(10, String::from("milk container"));
        let foam_container = RechargeableContainer::new(
            5,
            String::from("foam container"),
            ContainerRechargerController::new(Arc::new(milk_container)),
            1,
        );
        let cocoa_container = UnrechargeableContainer::new(5, String::from("container"));
        let water_container = NetworkRechargeableContainer::new(5, String::from("container"));

        let dispenser = Dispenser::new(0);
        let dispenser_result = dispenser.prepare_order(
            Order::new(1, coffee_units, cocoa_units, foam_units, water_units),
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
        use crate::coffee_maker::container::Container;
        use crate::coffee_maker::dispenser::Dispenser;
        use crate::coffee_maker::network_rechargeable_container::NetworkRechargeableContainer;
        use crate::coffee_maker::rechargeable_container::RechargeableContainer;
        use crate::coffee_maker::{
            container_rechargeable_controller::ContainerRechargerController,
            provider_container::ProviderContainer,
        };
        use crate::{
            coffee_maker::unrechargeable_container::UnrechargeableContainer, order::order::Order,
        };
        use std::sync::Arc;

        let coffee_units = 3;
        let cocoa_units = 3;
        let foam_units = 3;
        let water_units = 3;

        let grain_container = ProviderContainer::new(10, String::from("grain container"));
        let coffee_container = RechargeableContainer::new(
            5,
            String::from("coffee container"),
            ContainerRechargerController::new(Arc::new(grain_container)),
            1,
        );

        let milk_container = ProviderContainer::new(0, String::from("milk container"));
        let foam_container = RechargeableContainer::new(
            5,
            String::from("foam container"),
            ContainerRechargerController::new(Arc::new(milk_container)),
            1,
        );
        let _ = foam_container.extract(5);

        let cocoa_container = UnrechargeableContainer::new(5, String::from("container"));
        let water_container = NetworkRechargeableContainer::new(5, String::from("container"));

        let dispenser = Dispenser::new(0);
        let dispenser_result = dispenser.prepare_order(
            Order::new(1, coffee_units, cocoa_units, foam_units, water_units),
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
        use crate::coffee_maker::container::Container;
        use crate::coffee_maker::dispenser::Dispenser;
        use crate::coffee_maker::network_rechargeable_container::NetworkRechargeableContainer;
        use crate::coffee_maker::rechargeable_container::RechargeableContainer;
        use crate::coffee_maker::{
            container_rechargeable_controller::ContainerRechargerController,
            provider_container::ProviderContainer,
        };
        use crate::{
            coffee_maker::unrechargeable_container::UnrechargeableContainer, order::order::Order,
        };
        use std::sync::Arc;

        let coffee_units = 3;
        let cocoa_units = 3;
        let foam_units = 3;
        let water_units = 3;

        let grain_container = ProviderContainer::new(10, String::from("grain container"));
        let coffee_container = RechargeableContainer::new(
            5,
            String::from("coffee container"),
            ContainerRechargerController::new(Arc::new(grain_container)),
            1,
        );

        let milk_container = ProviderContainer::new(10, String::from("milk container"));
        let foam_container = RechargeableContainer::new(
            5,
            String::from("foam container"),
            ContainerRechargerController::new(Arc::new(milk_container)),
            1,
        );

        let cocoa_container = UnrechargeableContainer::new(5, String::from("container"));
        let water_container = NetworkRechargeableContainer::new(5, String::from("container"));
        let _ = water_container.extract(5);

        let dispenser = Dispenser::new(0);
        let dispenser_result = dispenser.prepare_order(
            Order::new(1, coffee_units, cocoa_units, foam_units, water_units),
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
        use crate::coffee_maker::container::Container;
        use crate::coffee_maker::dispenser::Dispenser;
        use crate::coffee_maker::network_rechargeable_container::NetworkRechargeableContainer;
        use crate::coffee_maker::rechargeable_container::RechargeableContainer;
        use crate::coffee_maker::{
            container_rechargeable_controller::ContainerRechargerController,
            provider_container::ProviderContainer,
        };
        use crate::{
            coffee_maker::unrechargeable_container::UnrechargeableContainer, order::order::Order,
        };
        use std::sync::Arc;

        let coffee_units = 3;
        let cocoa_units = 3;
        let foam_units = 3;
        let water_units = 3;

        let grain_container = ProviderContainer::new(10, String::from("grain container"));
        let coffee_container = RechargeableContainer::new(
            5,
            String::from("coffee container"),
            ContainerRechargerController::new(Arc::new(grain_container)),
            1,
        );
        let _ = coffee_container.extract(4);

        let milk_container = ProviderContainer::new(10, String::from("milk container"));
        let foam_container = RechargeableContainer::new(
            5,
            String::from("foam container"),
            ContainerRechargerController::new(Arc::new(milk_container)),
            1,
        );

        let cocoa_container = UnrechargeableContainer::new(5, String::from("container"));
        let water_container = NetworkRechargeableContainer::new(5, String::from("container"));
        let _ = water_container.extract(5);

        let dispenser = Dispenser::new(0);
        let dispenser_result = dispenser.prepare_order(
            Order::new(1, coffee_units, cocoa_units, foam_units, water_units),
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
        use crate::coffee_maker::container::Container;
        use crate::coffee_maker::dispenser::Dispenser;
        use crate::coffee_maker::network_rechargeable_container::NetworkRechargeableContainer;
        use crate::coffee_maker::rechargeable_container::RechargeableContainer;
        use crate::coffee_maker::{
            container_rechargeable_controller::ContainerRechargerController,
            provider_container::ProviderContainer,
        };
        use crate::{
            coffee_maker::unrechargeable_container::UnrechargeableContainer, order::order::Order,
        };
        use std::sync::Arc;

        let coffee_units = 3;
        let cocoa_units = 3;
        let foam_units = 3;
        let water_units = 3;

        let grain_container = ProviderContainer::new(10, String::from("grain container"));
        let coffee_container = RechargeableContainer::new(
            5,
            String::from("coffee container"),
            ContainerRechargerController::new(Arc::new(grain_container)),
            1,
        );
        
        let milk_container = ProviderContainer::new(10, String::from("milk container"));
        let foam_container = RechargeableContainer::new(
            5,
            String::from("foam container"),
            ContainerRechargerController::new(Arc::new(milk_container)),
            1,
        );
        let _ = foam_container.extract(4);

        let cocoa_container = UnrechargeableContainer::new(5, String::from("container"));
        let water_container = NetworkRechargeableContainer::new(5, String::from("container"));
        let _ = water_container.extract(5);

        let dispenser = Dispenser::new(0);
        let dispenser_result = dispenser.prepare_order(
            Order::new(1, coffee_units, cocoa_units, foam_units, water_units),
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
        use crate::coffee_maker::container::Container;
        use crate::coffee_maker::dispenser::Dispenser;
        use crate::coffee_maker::network_rechargeable_container::NetworkRechargeableContainer;
        use crate::coffee_maker::rechargeable_container::RechargeableContainer;
        use crate::coffee_maker::{
            container_rechargeable_controller::ContainerRechargerController,
            provider_container::ProviderContainer,
        };
        use crate::{
            coffee_maker::unrechargeable_container::UnrechargeableContainer, order::order::Order,
        };
        use std::sync::Arc;

        let cocoa_units = 3;
        let foam_units = 3;
        let water_units = 3;

        let grain_container = ProviderContainer::new(10, String::from("grain container"));
        let coffee_container = RechargeableContainer::new(
            5,
            String::from("coffee container"),
            ContainerRechargerController::new(Arc::new(grain_container)),
            1,
        );
        
        let milk_container = ProviderContainer::new(10, String::from("milk container"));
        let foam_container = RechargeableContainer::new(
            5,
            String::from("foam container"),
            ContainerRechargerController::new(Arc::new(milk_container)),
            1,
        );

        let cocoa_container = UnrechargeableContainer::new(5, String::from("container"));
        let water_container = NetworkRechargeableContainer::new(5, String::from("container"));
        let _ = water_container.extract(5);

        let dispenser = Dispenser::new(0);
        let dispenser_result = dispenser.prepare_order(
            Order::new(1, 0, cocoa_units, foam_units, water_units),
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
        use crate::coffee_maker::container::Container;
        use crate::coffee_maker::dispenser::Dispenser;
        use crate::coffee_maker::network_rechargeable_container::NetworkRechargeableContainer;
        use crate::coffee_maker::rechargeable_container::RechargeableContainer;
        use crate::coffee_maker::{
            container_rechargeable_controller::ContainerRechargerController,
            provider_container::ProviderContainer,
        };
        use crate::{
            coffee_maker::unrechargeable_container::UnrechargeableContainer, order::order::Order,
        };
        use std::sync::Arc;

        let coffee_units = 3;
        let foam_units = 3;
        let water_units = 3;

        let grain_container = ProviderContainer::new(10, String::from("grain container"));
        let coffee_container = RechargeableContainer::new(
            5,
            String::from("coffee container"),
            ContainerRechargerController::new(Arc::new(grain_container)),
            1,
        );
        
        let milk_container = ProviderContainer::new(10, String::from("milk container"));
        let foam_container = RechargeableContainer::new(
            5,
            String::from("foam container"),
            ContainerRechargerController::new(Arc::new(milk_container)),
            1,
        );

        let cocoa_container = UnrechargeableContainer::new(5, String::from("container"));
        let water_container = NetworkRechargeableContainer::new(5, String::from("container"));
        let _ = water_container.extract(5);

        let dispenser = Dispenser::new(0);
        let dispenser_result = dispenser.prepare_order(
            Order::new(1, coffee_units, 0, foam_units, water_units),
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
        use crate::coffee_maker::container::Container;
        use crate::coffee_maker::dispenser::Dispenser;
        use crate::coffee_maker::network_rechargeable_container::NetworkRechargeableContainer;
        use crate::coffee_maker::rechargeable_container::RechargeableContainer;
        use crate::coffee_maker::{
            container_rechargeable_controller::ContainerRechargerController,
            provider_container::ProviderContainer,
        };
        use crate::{
            coffee_maker::unrechargeable_container::UnrechargeableContainer, order::order::Order,
        };
        use std::sync::Arc;

        let coffee_units = 3;
        let cocoa_units = 3;
        let water_units = 3;

        let grain_container = ProviderContainer::new(10, String::from("grain container"));
        let coffee_container = RechargeableContainer::new(
            5,
            String::from("coffee container"),
            ContainerRechargerController::new(Arc::new(grain_container)),
            1,
        );
        
        let milk_container = ProviderContainer::new(10, String::from("milk container"));
        let foam_container = RechargeableContainer::new(
            5,
            String::from("foam container"),
            ContainerRechargerController::new(Arc::new(milk_container)),
            1,
        );

        let cocoa_container = UnrechargeableContainer::new(5, String::from("container"));
        let water_container = NetworkRechargeableContainer::new(5, String::from("container"));
        let _ = water_container.extract(5);

        let dispenser = Dispenser::new(0);
        let dispenser_result = dispenser.prepare_order(
            Order::new(1, coffee_units, cocoa_units, 0, water_units),
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
        use crate::coffee_maker::container::Container;
        use crate::coffee_maker::dispenser::Dispenser;
        use crate::coffee_maker::network_rechargeable_container::NetworkRechargeableContainer;
        use crate::coffee_maker::rechargeable_container::RechargeableContainer;
        use crate::coffee_maker::{
            container_rechargeable_controller::ContainerRechargerController,
            provider_container::ProviderContainer,
        };
        use crate::{
            coffee_maker::unrechargeable_container::UnrechargeableContainer, order::order::Order,
        };
        use std::sync::Arc;

        let coffee_units = 3;
        let cocoa_units = 3;
        let foam_units = 3;

        let grain_container = ProviderContainer::new(10, String::from("grain container"));
        let coffee_container = RechargeableContainer::new(
            5,
            String::from("coffee container"),
            ContainerRechargerController::new(Arc::new(grain_container)),
            1,
        );
        
        let milk_container = ProviderContainer::new(10, String::from("milk container"));
        let foam_container = RechargeableContainer::new(
            5,
            String::from("foam container"),
            ContainerRechargerController::new(Arc::new(milk_container)),
            1,
        );

        let cocoa_container = UnrechargeableContainer::new(5, String::from("container"));
        let water_container = NetworkRechargeableContainer::new(5, String::from("container"));
        let _ = water_container.extract(5);

        let dispenser = Dispenser::new(0);
        let dispenser_result = dispenser.prepare_order(
            Order::new(1, coffee_units, cocoa_units, foam_units, 0),
            &coffee_container,
            &foam_container,
            &water_container,
            &cocoa_container,
        );

        assert_eq!(dispenser_result, Ok(1));
    }
}
