fn show<T>(v: &[T]) -> () 
where T: std::fmt::Display {
	for i in v.iter() {
		println!("{}", i);
	}
}

fn main() {
	let t = vec![1, 22, 123, 11, 22, 2,3,4];
	show(&t);
}