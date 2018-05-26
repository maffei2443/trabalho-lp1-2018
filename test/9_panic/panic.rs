#[derive(Debug)]
struct Name {
	field: i32
}

use std::fs::File;

fn main() {
	let f = File::open("hello.txt");
  let v:Vec<i8> = vec![10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10];

	let file = match f {
		Ok(some) => {
      println!("Conseguiu le o arquivo!{:?}", some);
      let texto = String::new();
      some.read_to_end(&mut texto).expect("Falhou; nunca deve acontecer") ;
      println!("{}", texto);
    },
    Err(erro) => panic!("Erro ao abrir o arquivo hello.txt : {:?}", erro)
	};
}