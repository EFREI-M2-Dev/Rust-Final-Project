use crate::map;

use crate::map::generator::generate_map;
use crate::map::modifier::{add_base, add_random_elements};
use map::base::Base;
use map::TileType;

pub fn start_game() {
    let width = 30;
    let height = 30;
    let seed = 10;

    let mut map = generate_map(width, height, seed, vec![]);

    let base_position =
        Base::find_free_position(&map.grid).expect("Aucune place libre pour la base !");
    let mut base = Base::new(base_position.0, base_position.1);

    let modifiers = vec![
        add_base(&base),
        add_random_elements(TileType::Mineral, 0.01, seed),
        add_random_elements(TileType::Energy, 0.006, seed),
        add_random_elements(TileType::Interest, 0.003, seed),
    ];

    map = generate_map(width, height, seed, modifiers);
    map.add_robot(base.x, base.y, seed);

    println!("Base position: {:?}", base_position);

    loop {
        map.update_robots(&mut base);
        map.print();
        println!("=====================");
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
