pub fn example() {
  let tuple = (1, 2, 3);
  println!("tuple => {:?}", tuple);
  println!("tuple.0 => {}", tuple.0);

  let (a, b, c)= tuple;
  println!("tuple elements => {} {} {}", a, b, c);

  let mut array: [i32; 10] = [0; 10];
  println!("array => {:?}", array);

  array[0] = 1;
  array[3] = 4;
  array[9] = 10;
  println!("array => {:?}", array);

  println!("array[0] => {}", array[0]);

  // dynamic sized type (DST)
  let mut slice: &[i32] = &array[1..4];
  println!("slice => {:?}", slice);

  slice = &array[2..5];
  slice.iter().for_each(|x| println!("{} ", x));
  println!();

  let mut vec = Vec::new();
  vec.push(1);
  vec.push(2);
  println!("vec => {:?}", vec);

  let mut vec = vec![7, 8, 9];
  vec.push(10);
  println!("vec[0] => {}", vec[0]);
  println!("vec.pop() => {}", vec.pop().unwrap());
}