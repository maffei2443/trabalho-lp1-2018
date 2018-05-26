struct MyBox<T>(T);

impl<T> MyBox<T> {
	fn new(x: T)-> MyBox<T> {
		MyBox(x)
	}
}

fn main() {
	let x = 5;
	let k = Box::new(x);
	let y = MyBox::new(x);

	assert_eq!(5, *k);
	assert_eq!(5, x);
	// assert_eq!(5, *y);
	// println!("{:?}", y);
}