#[cfg(test)]
use mockall::automock;

use crate::statistics_checker::statistic::Statistic;

#[cfg_attr(test, automock)]
pub trait Container {
    fn extract(&self, extraction: u32) -> Result<u32, String>;

    fn get_statistics(&self) -> Statistic;

    fn amount_left_percentage_below_line(
        &self,
        amount_left: u32,
        max_capacity: u32,
        amount_percentage_alert: f32,
    ) -> bool {
        if max_capacity == 0 {
            false
        } else {
            amount_percentage_alert >= (amount_left / max_capacity) as f32
        }
    }

    fn check_alert_on_amount_left_percentage(
        &self,
        name: &String,
        amount_left: u32,
        max_capacity: u32,
        amount_percentage_alert: f32,
    ) -> bool {
        if self.amount_left_percentage_below_line(
            amount_left,
            max_capacity,
            amount_percentage_alert,
        ) {
            println!(
                "[{}] amount left percentage below {}%",
                name,
                amount_percentage_alert * 100_f32
            );
            return true;
        }
        false
    }
}
