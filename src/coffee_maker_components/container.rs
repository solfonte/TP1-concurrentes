use crate::statistics_checker::statistic::Statatistic;

pub trait Container {
    fn extract(&self, extraction: u32) -> Result<u32, &str>;
    fn get_statistics(&self) -> Statatistic;
}
