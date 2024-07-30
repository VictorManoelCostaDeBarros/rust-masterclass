use rpassword::prompt_password;
use std::io::Write;

pub fn show_menu(title: &str, items: &[&str], exit: bool) -> u32 {
  clean_screen();

  let complete: String = String::from("Masterclass Rust :: ") + title;
  println!("{}", complete);
  println!("{}", String::from("=").repeat(complete.len()));

  show_items(items);

  println!("{}", if exit { "* - Exit" } else { "* - Back" });
  print!("\nChoose one option: ");
  std::io::stdout().flush().unwrap();

  let mut line = String::new();
  std::io::stdin().read_line(&mut line).unwrap();

  let option: Result<u32, _> = line.trim().parse();

  match option {
    Ok(option) => option,
    _ => 0,
  }
}

fn show_items(items: &[&str]) {
  for (i, item) in items.iter().enumerate() {
    println!("{} - {}", i + 1, item)
  }
}

pub fn wait_enter() {
  prompt_password("Press enter to cotinue...").unwrap();
}

pub fn clean_screen() {
  print!("{esc}c", esc = 27 as char);
}