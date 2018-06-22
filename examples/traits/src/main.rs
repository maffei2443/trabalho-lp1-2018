extern crate summ;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let x = 123;
    x.summarize();
}
