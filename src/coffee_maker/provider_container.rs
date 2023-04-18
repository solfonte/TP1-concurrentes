use std::sync::{Arc, Condvar, Mutex};

use super::container::Container;


#[derive(Debug)]
pub struct System {
    amount: u32,
    busy: bool,
    is_on: bool,
}

pub struct ProviderContainer {
    /* cantidad actual, is_on, is_busy */
    pair: Arc<(Mutex<System>, Condvar)>,
    name: String,
}

impl Container for ProviderContainer {
    fn extract(&self, extraction: u32) -> Result<u32, &str> {
        let mut result: Result<u32, &str> = Err("No se pudo extraer del contenedor"); 
        if let Ok(guard) = self.pair.0.lock() {
            if let Ok(mut system) = self.pair.1.wait_while(guard, |state| {
                state.busy && state.is_on
            }) {
                println!("[Provider {}] {:?}", self.name, *system);

                (*system).busy = true;

                if (*system).amount >= extraction {
                    (*system).amount -= extraction;
                    result = Ok(extraction);
                } else {
                    result = Ok((*system).amount);
                    (*system).amount = 0;      
                }

                (*system).busy = false;
            }
        }
        self.pair.1.notify_all();
        result
    }
}

impl ProviderContainer {
    pub fn new(
        max_capacity: u32,
        name: String,
    ) -> Self {
        Self {
            pair: Arc::new((
                Mutex::new(System {
                    amount: max_capacity,
                    busy: false,
                    is_on: true,
                }),
                Condvar::new(),
            )),
            name,
        }
    }
}



/*
Condiciones:
    - si me alcanza -> saco
    - No me alcanza -> pido
                            -> puede recargar igual o menos de lo pedido -> recargo -> devuelvo Ok(extraccion)
                            -> no puede recargar -> devuelvo Ok(0)




1 gr de grain - 10 gr de cafe molido
*/


