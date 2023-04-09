#[derive(Debug)]
pub struct Order {
    order_number: u32,
    coffee_amount: u32,
    cocoa_amount: u32,
    milk_foam_amount: u32,
    water_amount: u32,
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
}
