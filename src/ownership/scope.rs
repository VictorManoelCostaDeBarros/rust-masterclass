// 1. Todo valor em Rust tem um dono.
// 2. Só pode ter um dono por vez.
// 3. Quando dono sair do escopo, valor será descartado.

pub fn basic_example() {
  {
      let s = String::from("Hello");
      println!("{}", s);
  }
  // println!("{}", s);
}

pub fn example_life_time() {
  let x;
  {
      let y = String::from("Scope internal");
      x = &y;
      println!("{} {}", x, y);
      println!("{}", x);
  }
}

pub fn example_move() {
  // Valor é alocado na stack
  let num1 = 10;
  let num2 = num1;
  println!("{} {}", num1, num2);

  // Valor é alocado na heap
  let s1 = String::from("Hello");

  // s1 foi movido para s2
  let s2 = s1;

  // println!("{}", s1);
  println!("{}", s2);
}

pub fn example_clone() {
  let s1 = String::from("Hello");

  // Clone precisa ser explicitamente chamado
  let s2 = s1.clone();

  println!("{} {}", s1, s2);
}

pub fn example_copy() {
  // Valores fixos são armazenados na stack e são copiados
  // Precisa implementar a trait Copy
  // i32, f64, bool, char, tuplas com tipos Copy
  let number_a = [1, 2, 3, 4, 5];
  let number_b = number_a;

  println!("{:?} {:?}", number_a, number_b);
}
