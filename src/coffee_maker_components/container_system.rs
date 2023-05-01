pub struct ContainerSystem {
    amount_left: u32,
    amount_consumed: u32,
    busy: bool,
    already_alerted_amount_percentage: bool,
}

impl ContainerSystem {
    pub fn new(amount: u32) -> Self {
        Self {
            amount_left: amount,
            amount_consumed: 0,
            busy: false,
            already_alerted_amount_percentage: false,
        }
    }

    pub fn set_already_alerted_amount_percentage(&mut self, state: bool) {
        self.already_alerted_amount_percentage = state;
    }

    pub fn set_busy(&mut self, busy: bool) {
        self.busy = busy;
    }

    pub fn is_busy(&self) -> bool {
        self.busy
    }

    pub fn get_amount_left(&self) -> u32 {
        self.amount_left
    }

    pub fn get_amount_consumed(&self) -> u32 {
        self.amount_consumed
    }

    pub fn extract(&mut self, extraction: u32) {
        self.amount_left -= extraction;
    }

    pub fn increase_amount_consumed(&mut self, consumed: u32) {
        self.amount_consumed += consumed;
    }

    pub fn recharge(&mut self, recharge: u32) {
        self.amount_left += recharge;
    }

    pub fn already_alerted_amount_percentage(&self) -> bool {
        self.already_alerted_amount_percentage
    }
}
