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

