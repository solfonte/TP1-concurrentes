use std::sync::Arc;

use super::{container::Container, container_controller::ContainerController};


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
