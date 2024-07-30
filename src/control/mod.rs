mod conditions;
mod loops;

use crate::utils::terminal::{show_menu, wait_enter, clean_screen};

pub fn execute() {
  loop {
    let itens = [
      "Conditions",
      "For - Range",
      "For - Array",
      "While",
      "Loop",
    ];
    let selected = show_menu("Tipos", &itens, false);
    clean_screen();
  
    match selected {
      1 => conditions::example(),
      2 => loops::example_for_range(),
      3 => loops::example_for_array(),
      4 => loops::example_while(),
      5 => loops::example_loop(),
      _ => break,
    }
  
    wait_enter();
  }
}