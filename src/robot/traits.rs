use crate::module::traits::Module;

pub trait Robot {
    fn name(&self) -> &str;
    fn battery(&self) -> f64;
    fn battery_capacity(&self) -> f64;
    fn consume_battery(&mut self, amount: f64);
    fn position(&self) -> (f64, f64);
    fn set_position(&mut self, x: f64, y: f64);
    fn perform_task(&mut self);
    fn robot_type(&self) -> &str;
    fn modules(&self) -> &Vec<Box<dyn Module>>;
    fn add_module(&mut self, module: Box<dyn Module>);
    fn remove_module(&mut self, module_name: &str);

    fn move_to(&mut self, x: f64, y: f64) {
        self.set_position(x, y);
        println!("{} se déplace vers ({}, {})", self.name(), x, y);
    }

    fn recharge(&mut self) {
        println!("{} se recharge...", self.name());
    }
}
