#[allow(clippy::comparison_chain)]

pub fn example() {
  let x = 10;
  let y = 5;

  if x > y {
    println!("x is greater than y");
  } else if x < y {
    println!("x is smaller than y")
  } else {
    println!("x is equal to y")
  }
}