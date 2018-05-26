fn main() {
    let mut pos = std::collections::HashMap::new();
    pos.insert(String::from("First"), 1);
    pos.insert(String::from("Second"), 2);
    println!("HashMap :: {:?}", pos);

    use std::collections::HashMap; // Nao eh incluido por padrao.
    
    let posicoes = vec![1,2,3];
    let nomes = vec!["First", "Second", "Third"];
    let best_pos: HashMap<_, _> = posicoes.iter().zip(nomes.iter()).collect();
    // Note: The type annotation HashMap<_, _> is needed here because it’s possible to collect into many different data structures, [https://doc.rust-lang.org/book/second-edition/ch08-03-hash-maps.html, 23_04_2018]
    println!("HashMap feito com os comandos :: ");
    println!("let posicoes = vec![1,2,3];");
    println!("let nomes = vec![\"First\", \"Second\", \"Third\"]");
    println!("let best_pos: HashMap<_, _> = posicoes.iter().zip(nomes.iter()).collect();");    
    println!("\n{:?}\n", best_pos);
}