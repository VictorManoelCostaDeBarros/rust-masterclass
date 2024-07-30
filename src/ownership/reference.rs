// 1. Em um dado momento, você pode ter uma referência mutável
// ou qualquer número de referências imutáveis.
// 2. As referências devem sempre ser válidas.

pub fn example_ref_immutable() {
  let text = String::from(
      "Rust is a systems programming language 
      which runs incredibly fast, prevents 
      segmentation and ensures concurrent security.",
  );

  let qtd_words = calculate_qtd_words(&text);
  println!("The text {} has {} words", text, qtd_words);
}

#[allow(clippy::ptr_arg)]
// Recebe uma referência para o texto (não toma posse)
// Borrows (empresta) o texto
fn calculate_qtd_words(text: &String) -> usize {
  text.split_whitespace().count()
  // texto é descartado (drop não é chamado porque não tem a posse - ownership)
}

pub fn example_ref_mutable_a() {
  let name = String::from("John");
  lastname(&name);
  println!("Name: {}", name);
}

#[allow(clippy::ptr_arg)]
// Não pode ser emprestado como mutável
fn lastname(text: &String) {
  // texto.push_str(" da Silva");
  println!("{}", text.len());
}

pub fn example_ref_mutable_b() {
  let mut name = String::from("John");
  lastname_mutable(&mut name);
  println!("Name: {}", name);

  let n1 = &mut name;
  println!("{}", n1);

  let n2 = &mut name;
  println!("{}", n2);
}

// Pode ser emprestado como mutável
fn lastname_mutable(text: &mut String) {
  text.push_str(" Doe");
}

pub fn example_ref_mutable_c() {
  let mut name = String::from("John");

  // imutável - sem problema
  let n1 = &name;
  let n2 = &name;

  println!("{} {}", n1, n2);

  // mutável - problema se trocar a ordem
  let n3 = &mut name;

  println!("{}", n3);
}

pub fn example_ref_pending() {
  // let aponta_para_nada = gerar_pendencia();
  // println!("Referência Pendente {}", aponta_para_nada);

  let without_pending = without_pending();
  println!("without Pending {}", without_pending);
}

// fn gerar_pendencia() -> &String {
//     let texto = String::from("Referência Pendente");
//     &texto // retorna uma referência que vai ser descartada
// }

#[allow(clippy::let_and_return)]
fn without_pending() -> String {
  let text = String::from("Without pending");
  text // retorna uma referência que será movida para a função chamadora
}
