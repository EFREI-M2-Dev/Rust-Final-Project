use crate::robot::traits::Robot;
use crate::module::traits::Module;

pub struct ConstructorRobot {
    name: String,
    battery: f64,
    position: (f64, f64),
    modules: Vec<Box<dyn Module>>,
}

impl ConstructorRobot {
    pub fn new(name: &str, battery_capacity: f64) -> Self {
        Self {
            name: name.to_string(),
            battery: battery_capacity,
            position: (0.0, 0.0),
            modules: Vec::new(),
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

    fn robot_type(&self) -> &str {
        "ConstructorRobot"
    }

    fn modules(&self) -> &Vec<Box<dyn Module>> {
        &self.modules
    }

    fn add_module(&mut self, module: Box<dyn Module>) {
        if module.compatible_robot() == self.robot_type() {
            println!("✅ {} ajoute le module {}", self.name, module.name());
            self.modules.push(module);
        } else {
            println!("❌ Impossible d'ajouter le module {} à {}, incompatibilité !", module.name(), self.name);
        }
    }

    fn remove_module(&mut self, module_name: &str) {
        self.modules.retain(|m| m.name() != module_name);
        println!("🗑️ {} a retiré le module {}", self.name, module_name);
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
