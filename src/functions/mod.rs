mod fns;
mod lambda;

use crate::utils::terminal::{show_menu, wait_enter, clean_screen};

pub fn execute() {
  loop {
    let itens = ["Basic", "Map", "Filter"];
    let selected = show_menu("Functions", &itens, false);

    clean_screen();

    match selected {
      1 => fns::example(),
      2 => lambda::example_map(),
      3 => lambda::example_filter(),
      _ => break,
    }

    wait_enter();
  }
}