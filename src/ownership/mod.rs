mod scope;
mod mechanism;
mod reference;
mod slice;

use crate::utils::terminal::{wait_enter, show_menu, clean_screen};

pub fn execute() {
    loop {
        let itens = [
            "Basic",
            "Life time",
            "Move",
            "Clone",
            "Copy",
            "Moving Possession #1",
            "Moving Possession #2",
            "Immutable Reference",
            "Mutable Reference #1",
            "Mutable Reference #2",
            "Mutable Reference #3",
            "Pending Reference",
            "Slice",
        ];
        let selecionado = show_menu("Ownership", &itens, false);

        clean_screen();

        match selecionado {
            1 => scope::basic_example(),
            2 => scope::example_life_time(),
            3 => scope::example_move(),
            4 => scope::example_clone(),
            5 => scope::example_copy(),
            6 => mechanism::example_a(),
            7 => mechanism::example_b(),
            8 => reference::example_ref_immutable(),
            9 => reference::example_ref_mutable_a(),
            10 => reference::example_ref_mutable_b(),
            11 => reference::example_ref_mutable_c(),
            12 => reference::example_ref_pending(),
            13 => slice::example(),
            _ => break,
        }

        wait_enter();
    }
}
