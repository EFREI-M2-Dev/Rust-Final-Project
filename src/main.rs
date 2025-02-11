mod robot;
mod resource;

use robot::{CollectorRobot, ExploratorRobot};
use crate::robot::traits::Robot;

use resource::{RadioactiveElement};
use crate::resource::traits::ResourceBehavior;


fn main() {
    let mut collector = CollectorRobot::new("Collecteur-01", 100.0);
    let mut explorator = ExploratorRobot::new("Explorateur-01", 120.0);

    collector.perform_task();
    collector.move_to(10.0, 20.0);

    explorator.perform_task();
    explorator.move_to(15.0, 25.0);

    let mut uranium = RadioactiveElement::new("Uranium-235", 100.0, 703.8);

   /*  let mut anti_hydrogen = Antimatter {
        name: "Antihydrogène".to_string(),
        amount: 5.0,
        containment_stability: 0.9,
    }; */

    println!("{:?}", uranium.name());
}