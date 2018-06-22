extern crate modulos;
use modulos::summarize::Summary;
fn main() {
	let x:i32 = 123;
	println!("{}", x.summarize());
}