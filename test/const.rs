const NUMERO: i32 = 123;
fn main() {
	println!("{:?}", NUMERO);
	let clo = || println!("Hellow!");
	clo();
}