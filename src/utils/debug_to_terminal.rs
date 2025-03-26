use super::config::Config;
use std::fs::OpenOptions;
use std::io::Write;

pub fn debug_to_terminal(msg: &str) {
    let config =
        Config::from_file("config.toml").expect("Erreur de chargement du fichier de configuration");
    let tty_path = &config.debug.tty_path;

    if let Ok(mut tty) = OpenOptions::new().write(true).open(tty_path) {
        let _ = writeln!(tty, "{}", msg);
    }
}
