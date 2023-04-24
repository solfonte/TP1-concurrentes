use std::sync::Arc;

use crate::coffee_maker::coffee_maker::CoffeeMaker;


pub struct StatisticsChecker {
    coffee_maker: Arc<CoffeeMaker>
}

impl StatisticsChecker {

    pub fn print_coffee_grain_statistics(&self) {
        let coffee_grain_container_statistics = self.coffee_maker.get_coffee_grain_statistics();
        println!("[Coffee grain container] amount left: {coffee_grain_container_statistics}");
    }
    
    pub fn print_cocoa_statistics(&self) {
        let cocoa_container_statistics = self.coffee_maker.get_cocoa_statistics();
        println!("[Cocoa container] amount left: {cocoa_container_statistics}");
    }

    pub fn print_milk_statistics(&self) {
        let milk_container_statistics = self.coffee_maker.get_milk_statistics();
        println!("[Milk container] amount left: {milk_container_statistics}");
    }
    
    pub fn print_water_statistics(&self) {
        let water_container_statistics = self.coffee_maker.get_water_statistics();
        println!("[Water container] amount left: {water_container_statistics}");
    }


    pub fn print_grounded_coffee_statistics(&self) {
        let ground_coffee_container_statistics = self.coffee_maker.get_ground_coffee_statistics();
        println!("[Ground coffee container] amount left: {ground_coffee_container_statistics}");
    }

    pub fn print_milk_foam_statistics(&self) {
        let milk_foam_container_statistics = self.coffee_maker.get_milk_foam_statistics();
        println!("[Milk foam container] amount left: {milk_foam_container_statistics}");
    }

    pub fn print_container_statistics(&self) -> bool {
        self.print_coffee_grain_statistics();
        self.print_cocoa_statistics();
        self.print_milk_statistics();
        self.print_grounded_coffee_statistics();
        self.print_milk_foam_statistics();
        return false;
    }

    pub fn new(coffee_maker: Arc<CoffeeMaker>) -> Self {
        Self {
            coffee_maker
        }
    }

}