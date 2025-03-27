mod map;
mod robot;
mod ui;
mod utils;

use crate::map::base::Base;
use crate::map::generator::generate_map;
use crate::map::modifier::{add_base, add_random_elements};
use crate::map::TileType;
use crate::ui::{draw_map, handle_input, setup_terminal, teardown_terminal};
use crate::utils::config::Config;
use std::io;

fn main() -> io::Result<()> {
    let config =
        Config::from_file("config.toml").expect("Erreur de chargement du fichier de configuration");

    let mut terminal = setup_terminal()?;

    let size = terminal.size()?;
    let mut width = size.width as usize;
    let mut height = size.height as usize;

    width = width - 3;
    height = height - 2;

    let seed = config.map.seed;

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

    loop {
        map.update_robots(&mut base);

        terminal.draw(|f| draw_map(f, &map)).unwrap();

        if handle_input() {
            break;
        }
    }

    teardown_terminal(&mut terminal)?;
    Ok(())
}
