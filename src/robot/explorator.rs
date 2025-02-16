use crate::robot::traits::Robot;
use crate::module::traits::Module;
use crate::map::BaseMap;

pub struct ExploratorRobot {
    name: String,
    battery: f64,
    position: (f64, f64),
    modules: Vec<Box<dyn Module>>,
}

impl ExploratorRobot {
    pub fn new(name: &str, battery_capacity: f64) -> Self {
        Self {
            name: name.to_string(),
            battery: battery_capacity,
            position: (0.0, 0.0),
            modules: Vec::new(),
        }
    }

    pub fn move_robot(&mut self, dx: isize, dy: isize, map: &mut BaseMap) {
        let new_x = (self.position.0 as isize + dx).clamp(0, map.width as isize - 1) as usize;
        let new_y = (self.position.1 as isize + dy).clamp(0, map.height as isize - 1) as usize;
        self.set_position(new_x as f64, new_y as f64);

        // map.reveal(new_x, new_y);
        map.reveal_area(new_x, new_y, 3);

        self.battery -= 5.0;
    }
}

impl Robot for ExploratorRobot {
    fn name(&self) -> &str {
        &self.name
    }

    fn battery(&self) -> f64 {
        self.battery
    }

    fn battery_capacity(&self) -> f64 {
        120.0
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
        "ExploratorRobot"
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
        if self.battery > 15.0 {
            println!("{} explore la zone à la recherche de nouveaux minerais...", self.name);
            self.consume_battery(15.0);
        } else {
            println!("{} n’a plus assez de batterie et doit se recharger.", self.name);
            self.recharge();
        }
    }
}
