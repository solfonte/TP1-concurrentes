use std::sync::Arc;
use super::{container::Container, container_controller::ContainerController};

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