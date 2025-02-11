pub trait ResourceBehavior {
    fn name(&self) -> &str;
    fn amount(&self) -> f64;

    fn extract(&mut self, amount: f64) -> f64;
    fn store(&mut self, amount: f64) -> bool;
}
