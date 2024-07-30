pub fn example() {
  basic_function();
  function_with_parameters(10, 20);

  let r = function_with_return();
  println!("Return #1 => {}", r);

  let r = function_with_parameters_and_return(10, 20);
  println!("Return #2 => {}", r);
}

fn basic_function() {
  println!("Basic function");
}

fn function_with_parameters(a: i32, b: i32) {
  println!("a => {}", a);
  println!("b => {}", b);
}

fn function_with_return() -> i32 {
  10
}

fn function_with_parameters_and_return(a: i32, b: i32) -> i32 {
  a + b
}