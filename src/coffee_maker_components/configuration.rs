use crate::order_taker_robot::file_reader::FileReader;
use serde_derive::Deserialize;

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
    pub amount_percentage_alert: f32,
}

pub struct SecurePowerState {
    pub busy: bool,
    pub on: bool,
}

pub struct SecureCounter {
    pub busy: bool,
    pub amount: u32,
}

pub struct ConfigurationReader {
    file_reader: FileReader,
}

impl ConfigurationReader {
    pub fn new(config_file: String) -> Self {
        Self {
            file_reader: FileReader::new(config_file),
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

#[cfg(test)]
mod test_configuration_reader {
    use crate::coffee_maker_components::configuration::ConfigurationReader;

    #[test]
    fn test01_when_taking_only_one_order_the_result_is_a_vector_with_one_order() {
        let configuration_reader = ConfigurationReader::new(String::from(
            "files/test_configuration_files/test_configurations.json",
        ));
        let result = configuration_reader.read_configuration();

        assert!(result.is_ok());

        let config = result.expect("Vector");

        assert_eq!(config.grain_capacity, 30);
        assert_eq!(config.ground_coffee_capacity, 30);
        assert_eq!(config.milk_capacity, 30);
        assert_eq!(config.milk_foam_capacity, 30);
        assert_eq!(config.cocoa_capacity, 30);
        assert_eq!(config.water_capacity, 30);
        assert_eq!(config.dispenser_amount, 10);
        assert_eq!(config.coffee_ground_recharge_rate, 2);
        assert_eq!(config.milk_foam_recharge_rate, 2);
        assert_eq!(config.heated_water_recharge_rate, 2);
    }
}
