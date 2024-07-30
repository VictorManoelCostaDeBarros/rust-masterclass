pub fn example() {
  let active: bool = true;
  println!("Boolean {}", active);

  let character: char = 'a';
  println!("Character {}", character);

  let name: &str = "Victor Manoel";
  println!("String {}", name);

  let mut name: String = String::from("John");
  name.push_str(" Doe");
  println!("String {}", name);

  // i8, i16, i32, i64, i128, isize
  // u8, u16, u32, u64, u128, usize
  let quantity: i32 = 10;
  println!("Int {}", quantity);

  // f32, f64
  let price: f64 = 10.99;
  println!("Float {}", price);
}