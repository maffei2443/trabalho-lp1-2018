fn main() {
    let mut s: std::string::String = std::string::String::new();
    s.push('b');
    println!("pushed 'b': {:?}", s);
    s.push_str("#tringui");
    println!("pushed \"#tringui\": {:?}", s);
    
    let st = "initial contents".to_string();
    let stt = String::from("initial contents");
    println!("{}", st);
    println!("{}", stt);
    println!("Acima, duas Strings iguais; muda apenas o estilo de inicializacao");
}