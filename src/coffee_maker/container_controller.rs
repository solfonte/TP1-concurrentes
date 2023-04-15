use super::container::Container;
pub struct ContainerController {
    container: Container,
    container_with_recharge: Container,
}

impl ContainerController {
    pub fn extract(&self, extraction: u32) -> Result<u32, String> {
        if let Ok(amount_extracted) = self.container.extract(extraction) {
            return Ok(amount_extracted.clone());
        }
        return Err(String::from("nose hubo error"));
    }

    pub fn new(container_capacity: u32, recharger_container_capacity: u32, name: String) -> Self {
        let container = Container::new(container_capacity, name);
        let container_with_recharge =
            Container::new(recharger_container_capacity, String::from("recharger"));
        Self {
            container,
            container_with_recharge,
        }
    }
}

/*

pub trait ContainerController{
    fn extract(&self, extraction: u32) -> Result<u32, String>;
}

pub struct ContainerRechargerController {
    container: Container,
    container_with_recharge: Container
}

impl ContainerRechargerController {
    pub fn new(container: Container, container_with_recharge: Container) -> Self {
        Self {
            container,
            container_with_recharge
        }
    }

    pub fn recharge(&self){}
}

impl ContainerController for ContainerRechargerController {

    fn extract(&self, extraction: u32) -> Result<u32, String> {
        if let Ok(amount_extracted) = self.container.extract(extraction) {
            return Ok(amount_extracted.clone())
        }
        return Err(String::from("nose hubo error"));
    }

}

*/
