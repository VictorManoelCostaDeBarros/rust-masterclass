pub fn example_for_range() {
  for a in 1..5 {
    println!("a => {}", a);
  }

  for b in 1..=5 {
    println!("a => {}", b);
  }

  for c in (1..5).rev() {
    println!("a => {}", c);
  }

  for d in (1..=10).step_by(2) {
    println!("a => {}", d);
  }
}

pub fn example_for_array() {
  let array = [1, 2, 3, 4, 5];

  for i in 0..array.len() {
    println!("array[{}]", i);
  }

  for value in array {
    println!("value => {}", value);
  }

  for (i, value) in array.iter().enumerate() {
    println!("array[{}] => {}", i, value)
  }
}

pub fn example_while() {
  let mut a = 1;

  while a <= 5 {
    println!("a => {}", a);
    a += 1;
  }
}

pub fn example_loop() {
  let mut b = 1;

  loop {
    println!("b => {}", b);
    b += 1;

    if b > 5 {
      break;
    }
  }
}