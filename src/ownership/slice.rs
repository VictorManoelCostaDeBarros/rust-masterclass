// Os Slice permitem fazer referência a uma sequência contígua
// de elementos em uma coleção, em vez de à coleção inteira.
// O Slice é uma espécie de referência, portanto não possui ownership.

pub fn example() {
  let text = String::from("Rust is mordem programing language");

  let words = first_word_with('l', &text);
  println!("The words is '{}'", words);
}

// Primeira palavra iniciada pela letra informada
fn first_word_with(word: char, slice: &str) -> &str {
  let bytes = slice.as_bytes();
  for (i, &item) in bytes.iter().enumerate() {
      if item == word as u8 {
          return slice[i..].split_whitespace().next().unwrap();
      }
  }
  slice
}
