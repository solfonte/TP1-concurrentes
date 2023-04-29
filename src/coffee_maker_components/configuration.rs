use serde_derive::Deserialize;
use crate::order_taker_robot::file_reader::FileReader;


#[derive(Debug, Deserialize)]
pub struct CoffeeMakerConfiguration {
    pub grain_capacity: u32,
    pub ground_coffee_capacity: u32, 
    pub milk_capacity: u32, 
    pub milk_foam_capacity: u32, 
    pub cocoa_capacity: u32, 
    pub water_capacity: u32, 
    pub dispenser_amount: u32,
    pub coffee_ground_recharge_rate: u32,
    pub milk_foam_recharge_rate: u32,
    pub heated_water_recharge_rate: u32,
}


pub struct ConfigurationReader {
    file_reader: FileReader,
}

impl ConfigurationReader {

    pub fn new(config_file: String) -> Self {
        Self {
            file_reader: FileReader::new(config_file)
        }
    }

    pub fn read_configuration(&self) -> Result<CoffeeMakerConfiguration, String> {
        match self.file_reader.read() {
            Ok(config) => {
                let stream = serde_json::from_str::<CoffeeMakerConfiguration>(&config);
                match stream {
                    Ok(coffe_maker_config) => Ok(coffe_maker_config),
                    Err(msg) => Err(msg.to_string()),
                }
            }
            Err(error_msg) => Err(error_msg),
        }
    }
}