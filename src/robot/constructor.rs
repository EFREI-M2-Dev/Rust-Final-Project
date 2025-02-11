use crate::robot::traits::Robot;

pub struct ConstructorRobot {
    name: String,
    battery: f64,
    position: (f64, f64),
}

impl ConstructorRobot {
    pub fn new(name: &str, battery_capacity: f64) -> Self {
        Self {
            name: name.to_string(),
            battery: battery_capacity,
            position: (0.0, 0.0),
        }
    }
}

impl Robot for ConstructorRobot {
    fn name(&self) -> &str {
        &self.name
    }

    fn battery(&self) -> f64 {
        self.battery
    }

    fn battery_capacity(&self) -> f64 {
        100.0
    }

    fn consume_battery(&mut self, amount: f64) {
        if self.battery >= amount {
            self.battery -= amount;
        } else {
            self.battery = 0.0;
        }
    }

    fn position(&self) -> (f64, f64) {
        self.position
    }

    fn set_position(&mut self, x: f64, y: f64) {
        self.position = (x, y);
    }

    fn modules(&self) -> Vec<String> {
        vec!["Boîte à outils".to_string()]
    }

    fn perform_task(&mut self) {
        if self.battery > 10.0 {
            println!("{} contruit quelque chose...", self.name);
            self.consume_battery(10.0);
        } else {
            println!("{} n’a plus assez de batterie et doit se recharger.", self.name);
            self.recharge();
        }
    }
}
