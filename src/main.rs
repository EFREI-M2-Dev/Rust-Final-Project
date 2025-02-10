mod robot;

use robot::{CollectorRobot, ExploratorRobot};
use crate::robot::traits::Robot;

fn main() {
    let mut collector = CollectorRobot::new("Collecteur-01", 100.0);
    let mut explorator = ExploratorRobot::new("Explorateur-01", 120.0);

    collector.perform_task();
    collector.move_to(10.0, 20.0);

    explorator.perform_task();
    explorator.move_to(15.0, 25.0);
}