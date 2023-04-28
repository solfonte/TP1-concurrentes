extern crate serde;
use serde_derive::Deserialize;


#[derive(Debug, Deserialize)]
pub struct Order {
    pub order_number: u32,
    pub coffee_amount: u32,
    pub cocoa_amount: u32,
    pub milk_foam_amount: u32,
    pub water_amount: u32,
}

impl Order {
    pub fn new(
        order_number: u32,
        coffee_amount: u32,
        cocoa_amount: u32,
        milk_foam_amount: u32,
        water_amount: u32,
    ) -> Self {
        Self {
            order_number,
            coffee_amount,
            cocoa_amount,
            milk_foam_amount,
            water_amount,
        }
    }

    pub fn get_order_number(&self) -> u32 {
        self.order_number
    }

    pub fn get_coffee_amount(&self) -> u32 {
        self.coffee_amount
    }

    pub fn get_cocoa_amount(&self) -> u32 {
        self.cocoa_amount
    }

    pub fn get_milk_foam_amount(&self) -> u32 {
        self.milk_foam_amount
    }

    pub fn get_water_amount(&self) -> u32 {
        self.water_amount
    }
}
