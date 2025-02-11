use crate::module::traits::Module;

pub struct CargoExpand;

impl Module for CargoExpand {
    fn name(&self) -> &str { "Cargo Expand" }
    fn compatible_robot(&self) -> &str { "CollectorRobot" }
    fn activate(&self) {
        println!("📦 Cargo Expand activé ! Capacité de stockage augmentée de 50%.");
    }
}
