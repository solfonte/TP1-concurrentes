use std::sync::Arc;

use super::{container::Container, provider_container::ProviderContainer};

pub struct ContainerRechargerController {
    recharger_container: Arc<ProviderContainer>,
    name: String,
}

impl ContainerRechargerController {
    pub fn new(recharger_container: Arc<ProviderContainer>, name: String) -> Self {
        Self {
            recharger_container,
            name,
        }
    }

    pub fn recharge(&self, amount_to_extract: u32) -> Result<u32, &str> {
        println!("[Controller {}] recharging", self.name);
        self.recharger_container.extract(amount_to_extract)
    }
}
