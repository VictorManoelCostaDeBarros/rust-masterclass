mod first;
mod variables;
mod operators;

use crate::utils::terminal::{show_menu, wait_enter, clean_screen};

pub fn execute() {
  loop {
    let items = [
      "First example",
      "Variables - Immutable",
      "Variables - Mmutable",
      "Variables - Constants",
      "Variables - Shadowing",
      "Operators - Arithmetic",
      "Operators - Relational",
      "Operators - Logicians"
    ];

    let selected = show_menu("Fundamentos", &items, false);

    clean_screen();

    match selected {
      1 => first::example(),
      2 => variables::immutable(),
      3 => variables::mmutable(),
      4 => variables::constants(),
      5 => variables::shadowing(),
      6 => operators::arithmetic(),
      7 => operators::relational(),
      8 => operators::logicians(),
      _ => break,
    }

    wait_enter();
  }
}