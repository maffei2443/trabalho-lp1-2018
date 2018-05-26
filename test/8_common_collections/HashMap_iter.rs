use std::collections::HashMap;
fn main() {
    let mut map = HashMap::new();
    map.insert("First", 1);
    map.insert("Second", 2);
    println!("Iterando com 'for' e (chave, valor):");
    for (chave, valor) in &map {
        println!("{} {}", chave, valor);
    }
    for (chave, valor) in map.iter() {
        println!("{} {}", chave, valor);
    }
    let x = map.iter();
    println!("\nPrint de um iterador do map:");
    println!("{:?}", x);
}