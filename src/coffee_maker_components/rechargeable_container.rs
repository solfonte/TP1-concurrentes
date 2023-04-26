use std::sync::{Arc, Condvar, Mutex};

use crate::statistics_checker::statistic::Statatistic;

use super::{
    container::Container, container_rechargeable_controller::ContainerRechargerController,
};

#[derive(Debug)]
pub struct System {
    max_capacity: u32,
    amount: u32,
    amount_consumed: u32,
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
            if let Ok(mut system) = self
                .pair
                .1
                .wait_while(guard, |state| state.busy && state.is_on)
            {
                result = self.extract_amount(&mut system, extraction);
            }
        }
        self.pair.1.notify_all();
        result
    }

    fn get_statistics(&self) -> Statatistic {
        let mut amount_left = 0;
        let mut amount_consumed = 0;
        if let Ok(guard) = self.pair.0.lock() {
            if let Ok(mut system) = self.pair.1.wait_while(guard, |state| state.busy) {
                system.busy = true;
                amount_left = system.amount;
                amount_consumed = system.amount_consumed;
                system.busy = false;
            }
        }
        Statatistic {
            amount_left,
            amount_consumed,
            container: String::from(&self.name),
        }
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
                    amount_consumed: 0,
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

    pub fn extract_amount(&self, system: &mut System, extraction: u32) -> Result<u32, &str> {
       
        system.busy = true;

        if system.amount < extraction {
            let amount_to_recharge = (system.max_capacity - system.amount) / self.recharging_rate;
            let recharging_result = self.recharger_controller.recharge(amount_to_recharge);
            if let Ok(amount_returned) = recharging_result {
                println!("[CONTAINER COULD RECHARGE!]");
                system.amount += amount_returned * self.recharging_rate;
            }
        }

        let result = if system.amount >= extraction {
            system.amount -= extraction;
            system.amount_consumed += extraction;
            Ok(extraction)
        } else {
            Ok(0)
        };

        system.busy = false;
        result
    }
}


