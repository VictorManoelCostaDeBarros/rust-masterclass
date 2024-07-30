pub fn immutable() {
  let x = 5;

  println!("x => {}", x);
}

pub fn mmutable() {
  let mut x = 10;
  let y = x;
  println!("x, y => {} {}", x, y);

  x = 15;
  println!("x, y => {} {}", x, y);
}

pub fn constants() {
  const Z: i32 = 20;
  println!("Value of Z is: {}", Z);
}

pub fn shadowing() {
  let a = 25;
  println!("Value of a is: {}", a);

  let a = "Text";
  println!("Value of a is: {}", a);

  let a = [1, 2, 3, 4, 5];
  println!("Value of a is: {:?}", a);
}
