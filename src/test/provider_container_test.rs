
mod provider_container_test {
    
    #[test]
    fn test01_when_there_are_two_units_available_then_extracting_cero_is_possible_the_extraction_equals_cero() {
        use crate::coffee_maker::provider_container::ProviderContainer;
        use crate::coffee_maker::container::Container;
        
        let container = ProviderContainer::new(2, String::from("Container"));
        let extraction = container.extract(0);
        
        assert_eq!(extraction, Ok(0))
    }

    #[test]
    fn test02_when_there_are_two_units_available_then_extracting_one_is_possible_the_extraction_equals_one() {
        use crate::coffee_maker::provider_container::ProviderContainer;
        use crate::coffee_maker::container::Container;
        
        let container = ProviderContainer::new(2, String::from("Container"));
        let extraction = container.extract(1);
        
        assert_eq!(extraction, Ok(1))
    }

    #[test]
    fn test03_when_there_are_two_units_available_then_extracting_two_is_possible_the_extraction_equals_two() {
        use crate::coffee_maker::provider_container::ProviderContainer;
        use crate::coffee_maker::container::Container;
        
        let container = ProviderContainer::new(2, String::from("Container"));
        let extraction = container.extract(2);
        
        assert_eq!(extraction, Ok(2))
    }

    #[test]
    fn test04_when_there_are_two_units_available_then_extracting_three_is_not_possible_the_extraction_equals_two() {
        use crate::coffee_maker::provider_container::ProviderContainer;
        use crate::coffee_maker::container::Container;
        
        let container = ProviderContainer::new(2, String::from("Container"));
        let extraction = container.extract(3);
        
        assert_eq!(extraction, Ok(2))
    }
}