use std::sync::{Arc, Condvar, Mutex};

use super::{
    container::Container, container_rechargeable_controller::ContainerRechargerController,
};

#[derive(Debug)]
pub struct System {
    max_capacity: u32,
    amount: u32,
    busy: bool,
    is_on: bool,
}

pub struct RechargeableContainer {
    /* cantidad actual, is_on, is_busy */
    pair: Arc<(Mutex<System>, Condvar)>,
    name: String,
    recharger_controller: ContainerRechargerController,
    recharging_rate: u32,
}

impl Container for RechargeableContainer {
    fn extract(&self, extraction: u32) -> Result<u32, &str> {
        let mut result: Result<u32, &str> = Err("No se pudo extraer del contenedor"); 
        if let Ok(guard) = self.pair.0.lock() {
            if let Ok(mut system) = self.pair.1.wait_while(guard, |state| {
                state.busy && state.is_on
            }) {
                
                (*system).busy = true;
                
                if (*system).amount < extraction {
                    let amount_to_recharge = ((*system).max_capacity  - (*system).amount) / self.recharging_rate;
                    let recharging_result = self.recharger_controller.recharge(amount_to_recharge);
                    if let Ok(amount_returned) = recharging_result {
                        println!("[CONTAINER COULD RECHARGE!]");
                        (*system).amount += amount_returned * self.recharging_rate;
                    }    
                }
                
                if (*system).amount >= extraction {
                    (*system).amount -= extraction;
                    result = Ok(extraction);
                } else {
                    result = Ok(0);
                }
                
                println!("[container {}] {:?}", self.name, *system);
                (*system).busy = false;
            }
        }
        self.pair.1.notify_all();
        result
    }

    fn amount_left(&self) -> u32 {
        let mut amount_left = 0;
        if let Ok(guard) = self.pair.0.lock() {
            if let Ok(mut system) = self.pair.1.wait_while(guard, |state| {
                state.busy
            }) {
                (*system).busy = true;
                amount_left = (*system).amount;
                (*system).busy = false;
            }
        }
        amount_left
    }
}

impl RechargeableContainer {
    pub fn new(
        max_capacity: u32,
        name: String,
        recharger_controller: ContainerRechargerController,
        recharging_rate: u32,

    ) -> Self {
        Self {
            pair: Arc::new((
                Mutex::new(System {
                    max_capacity,
                    amount: max_capacity,
                    busy: false,
                    is_on: true,
                }),
                Condvar::new(),
            )),
            name,
            recharger_controller,
            recharging_rate,

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


