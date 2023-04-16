use std::sync::Arc;

use super::container::Container;
pub trait ContainerController {
    fn turn_on(&self) -> Result<u32, String>;

    fn container_statistics(&self, container: &Container) -> (u32, u32) {
        let (max_capacity, resource_available) = container.get_resource_info();
        /*
        if (check condicion de X%){
            print
        }*/

        (resource_available, max_capacity)
    }
}

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

pub struct NormalContainerController {
    container: Arc<Container>,
}

impl NormalContainerController {
    pub fn new(container: Arc<Container>) -> Self {
        Self { container }
    }
}

impl ContainerController for NormalContainerController {
    fn turn_on(&self) -> Result<u32, String> {
        // Check cada cierta cantidad de tiempo las cantidades como para ver si hay que imprimir

        Ok(0)
    }
}

pub struct WaterNetworkController {
    water_container: Arc<Container>,
}

impl WaterNetworkController {
    pub fn new(water_container: Arc<Container>) -> Self {
        Self { water_container }
    }
}

impl ContainerController for WaterNetworkController {
    fn turn_on(&self) -> Result<u32, String> {
        // Check cada cierta cantidad de tiempo las cantidades como para ver si hay que imprimir
        // Check cada cierta cantidad de tiempo las cantidades como para ver si hay que recargar
        //TODO: poner el criterio de carga

        Ok(0)
    }
}
