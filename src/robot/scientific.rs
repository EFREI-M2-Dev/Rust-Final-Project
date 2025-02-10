use crate::robot::traits::Robot;

pub struct ScientificRobot {
    pub name: String,
    
}

impl Robot for ScientificRobot {
    fn name(&self) -> &str {
        &self.name
    }

    fn battery(&self) -> f64 {
        0.0
    }

    fn consume_battery(&mut self, _amount: f64) {
        // Do nothing
    }

    fn battery_capacity(&self) -> f64 {
        0.0
    }

    fn position(&self) -> (f64, f64) {
        (0.0, 0.0)
    }

    fn set_position(&mut self, _x: f64, _y: f64) {
        // Do nothing
    }

    fn modules(&self) -> Vec<String> {
        vec![]
    }

    fn perform_task(&mut self) {
       println!("{} is doing scientific research", self.name);
    }
}
