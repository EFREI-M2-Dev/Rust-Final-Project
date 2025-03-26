use std::fs::OpenOptions;
use std::io::Write;

pub fn debug_to_terminal(msg: &str) {
    let tty_path = "/dev/ttys009";
    if let Ok(mut tty) = OpenOptions::new().write(true).open(tty_path) {
        let _ = writeln!(tty, "{}", msg);
    }
}
