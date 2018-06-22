fn name(arg: &str){
    println!("{:?}", arg);
}

fn main() {
    let s = "hellow, World";
    name(s);
    println!("{:?}", drop(s));
    let x = s;
}