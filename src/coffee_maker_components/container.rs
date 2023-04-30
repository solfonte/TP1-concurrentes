#[cfg(test)]
use mockall::automock;

use crate::statistics_checker::statistic::Statistic;

#[cfg_attr(test, automock)]
pub trait Container {
    fn extract(&self, extraction: u32) -> Result<u32, String>;
    fn get_statistics(&self) -> Statistic;
}
