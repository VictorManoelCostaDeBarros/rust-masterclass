mod basic;
mod sequential;
mod custom;

use crate::utils::terminal::{show_menu, wait_enter, clean_screen};

pub fn execute() {
  loop {
    let itens = [
      "Basic",
      "Sequential",
      "Custom - Struct",
      "Custom - Enums"
    ];
    let selected = show_menu("Tipos", &itens, false);
    clean_screen();
  
    match selected {
      1 => basic::example(),
      2 => sequential::example(),
      3 => custom::example_struct(),
      4 => custom::example_enum(),
      _ => break,
    }
  
    wait_enter();
  }
}