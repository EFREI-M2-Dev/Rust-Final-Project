pub trait Robot {
    fn name(&self) -> &str;
    fn battery(&self) -> f64;
    fn battery_capacity(&self) -> f64;
    fn consume_battery(&mut self, amount: f64);
    fn position(&self) -> (f64, f64);
    fn set_position(&mut self, x: f64, y: f64);
    fn modules(&self) -> Vec<String>;
    fn perform_task(&mut self);

    fn move_to(&mut self, x: f64, y: f64) {
        self.set_position(x, y);
        println!("{} se déplace vers ({}, {})", self.name(), x, y);
    }

    fn recharge(&mut self) {
        println!("{} se recharge...", self.name());
    }
}
