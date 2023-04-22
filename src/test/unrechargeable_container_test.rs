mod unrechargeable_container_test {
    
    #[test]
    fn test01_when_there_are_two_units_available_then_extracting_cero_is_possible_the_extraction_equals_cero() {
        use crate::coffee_maker::unrechargeable_container::UnrechargeableContainer;
        use crate::coffee_maker::container::Container;
    
        let container = UnrechargeableContainer::new(2, String::from("unit container"));
        let extraction = container.extract(0);
    
        assert_eq!(extraction, Ok(0))
    }
    
    #[test]
    fn test02_when_there_are_two_units_available_then_extracting_one_is_possible_the_extraction_equals_one() {
        use crate::coffee_maker::unrechargeable_container::UnrechargeableContainer;
        use crate::coffee_maker::container::Container;
    
        let container = UnrechargeableContainer::new(2, String::from("unit container"));
        let extraction = container.extract(1);
    
        assert_eq!(extraction, Ok(1))
    }
    
    #[test]
    fn test03_when_there_are_two_units_available_then_extracting_two_is_possible_the_extraction_equals_two() {
        use crate::coffee_maker::unrechargeable_container::UnrechargeableContainer;
        use crate::coffee_maker::container::Container;
    
        let container = UnrechargeableContainer::new(2, String::from("unit container"));
        let extraction = container.extract(2);
    
        assert_eq!(extraction, Ok(2))
    }
    
    
    #[test]
    fn test04_when_there_are_two_units_available_then_extracting_three_is_not_possible_the_extraction_equals_cero() {
        use crate::coffee_maker::unrechargeable_container::UnrechargeableContainer;
        use crate::coffee_maker::container::Container;
    
        let container = UnrechargeableContainer::new(2, String::from("unit container"));
        let extraction = container.extract(3);
    
        assert_eq!(extraction, Ok(0))
    }
    #[test]
    fn test05_when_there_are_two_units_available_then_extracting_cero_leaves_an_amount_of_two_units_left_available() {
        use crate::coffee_maker::unrechargeable_container::UnrechargeableContainer;
        use crate::coffee_maker::container::Container;
    
        let container = UnrechargeableContainer::new(2, String::from("unit container"));
        let _ = container.extract(0);
        let amount_left = container.amount_left();
    
        assert_eq!(amount_left, 2)
    }
    
    #[test]
    fn test06_when_there_are_two_units_available_then_extracting_one_leaves_an_amount_of_one_unit_left_available() {
        use crate::coffee_maker::unrechargeable_container::UnrechargeableContainer;
        use crate::coffee_maker::container::Container;
    
        let container = UnrechargeableContainer::new(2, String::from("unit container"));
        let _ = container.extract(1);
        let amount_left = container.amount_left();
    
        assert_eq!(amount_left, 1)
    }
    
    #[test]
    fn test07_when_there_are_two_units_available_then_extracting_two_leaves_an_amount_of_cero_units_left_available() {
        use crate::coffee_maker::unrechargeable_container::UnrechargeableContainer;
        use crate::coffee_maker::container::Container;
    
        let container = UnrechargeableContainer::new(2, String::from("unit container"));
        let _ = container.extract(2);
        let amount_left = container.amount_left();
    
        assert_eq!(amount_left, 0);
    }
    
    
    #[test]
    fn test08_when_there_are_two_units_available_then_extracting_three_leaves_an_amount_of_two_units_left_available() {
        use crate::coffee_maker::unrechargeable_container::UnrechargeableContainer;
        use crate::coffee_maker::container::Container;
    
        let container = UnrechargeableContainer::new(2, String::from("unit container"));
        let _ = container.extract(3);
        let amount_left = container.amount_left();
    
        assert_eq!(amount_left, 2)
    }

}
