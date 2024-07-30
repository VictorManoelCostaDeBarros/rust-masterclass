mod fundamentals;
mod types;
mod utils;
mod control;
mod functions;
mod ownership;

use std::process::exit;

use utils::terminal::{clean_screen, show_menu};

fn main() {
    loop {
        let items = ["Fundamentals", "Types", "Control", "Funtions", "Ownership"];
    
        let selected = show_menu("Principal", &items, true);

        clean_screen();

        match selected {
            1 => fundamentals::execute(),
            2 => types::execute(),
            3 => control::execute(),
            4 => functions::execute(),
            5 => ownership::execute(),
            _ => exit(0),
        }
    }

}