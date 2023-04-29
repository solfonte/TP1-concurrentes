use std::sync::Arc;

use crate::coffee_maker_components::coffee_maker::CoffeeMaker;

pub struct StatisticsChecker {
    coffee_maker: Arc<CoffeeMaker>,
}

impl StatisticsChecker {

    pub fn print_statistics(&self) -> bool {
        let statistics = self.coffee_maker.get_containers_statistics();
        
        for stat in statistics.0 {
            println!(
                "[{}] amount left: {}, amount consumed: {}, ",
                stat.container, stat.amount_left, stat.amount_consumed
            );
        }

        println!("Amount of prepared orders: {}", statistics.1);

        if !statistics.2 {
            println!("Coffee machine is off");
        }
        
        statistics.2   
    }

    pub fn new(coffee_maker: Arc<CoffeeMaker>) -> Self {
        Self { coffee_maker }
    }

    
}
