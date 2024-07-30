pub fn example_a() {
  let name = String::from("John");
  show_name(name);

  // println!("{}", name);

  let age = 30;
  show_age(age);

  println!("{}", age);
}

// A referência é movida para o argumento
// que toma posse do valor
fn show_name(name: String) {
  println!("{}", name);
} // name é descartado (drop é chamado)

// Recebe uma copia do valor (não toma posse)
fn show_age(age: i32) {
  println!("{}", age);
}

pub fn example_b() {
  let name = new_name();
  println!("{}", name);

  let (name, t) = calculate_size(name);
  println!("{} tem tamanho {}", name, t);
}

#[allow(clippy::let_and_return)]
fn new_name() -> String {
  let name = String::from("Maria");
  name // posse é transferida para a função chamadora
}

// Recebe a posse do valor (ownership)
fn calculate_size(name: String) -> (String, usize) {
  let size = name.len();
  (name, size) // Devolve a posse do valor para a função chamadora
}
