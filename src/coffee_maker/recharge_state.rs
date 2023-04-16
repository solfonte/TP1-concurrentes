use super::container::Container;
use std::sync::Arc;

pub struct RechargeState {
    is_rechargeable: bool,
    recharger_container: Option<Arc<Container>>,
}

impl RechargeState {
    pub fn new(is_rechargeable: bool, recharger_container: Option<Arc<Container>>) -> Self {
        Self {
            is_rechargeable,
            recharger_container,
        }
    }

    pub fn can_recharge(&mut self) -> bool {
        self.is_rechargeable
    }

    pub fn turn_unrechargeable(&mut self) {
        self.is_rechargeable = false;
    }
}
