use std::sync::Arc;

use super::{container::Container, provider_container::ProviderContainer};

pub struct ContainerRechargerController {
    recharger_container: Arc<ProviderContainer>,
}

impl ContainerRechargerController {
    pub fn new(recharger_container: Arc<ProviderContainer>) -> Self {
        Self {
            recharger_container,
        }
    }

    pub fn recharge(&self, amount_to_extract: u32) -> Result<u32, String> {
        self.recharger_container.extract(amount_to_extract)
    }
}
