use std::sync::Arc;

use super::unrechargeable_container::UnrechargeableContainer;

pub struct ContainerRechargerController {
    recharger_container: Arc<UnrechargeableContainer>,
}

impl ContainerRechargerController {
    pub fn new(recharger_container: Arc<UnrechargeableContainer>) -> Self {
        Self {
            recharger_container,
        }
    }

    pub fn recharge(&self) {}
}
