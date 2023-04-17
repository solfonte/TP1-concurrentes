use std::sync::Arc;

use super::container::Container;

pub struct ContainerRechargerController {
    recharger_container: Arc<Container>,
}

impl ContainerRechargerController {
    pub fn new(recharger_container: Arc<Container>) -> Self {
        Self {
            recharger_container,
        }
    }

    pub fn recharge(&self, container: &mut Container) {}
}
