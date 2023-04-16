use std::sync::Arc;

use super::{container::Container, container_controller::ContainerController};

pub struct ContainerRechargerController {
    container: Arc<Container>,
    recharger_container: Arc<Container>,
}

impl ContainerRechargerController {
    pub fn new(container: Arc<Container>, recharger_container: Arc<Container>) -> Self {
        Self {
            container,
            recharger_container,
        }
    }

    pub fn check_container_disponibility(
        &self,
        max_capacity: u32,
        available_resource: u32,
    ) -> Result<u32, String> {
        Ok(0)
    }

    pub fn recharge(&self) {}
}

impl ContainerController for ContainerRechargerController {
    fn turn_on(&self) -> Result<u32, String> {
        // Check cada cierta cantidad de tiempo las cantidades como para ver si hay que imprimir
        let (max_capacity, resource_available) = self.container_statistics(&self.container);
        self.check_container_disponibility(max_capacity, resource_available);
        Ok(0)
    }
}