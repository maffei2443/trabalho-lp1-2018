// struct genérica sobre um tipo T
struct Point<T> {
	x: T,
	y: T,
}

// exemplo de métodos apenas para quando  T é i32
impl Point<i32> {
	fn isInt(&self) -> bool {
		true
	}
}


impl<T> Point<T> {
	fn new(x: T, y: T) -> Point<T> {
		Point {
			x,
			y
		}
	}
}


fn main() {
	let int = Point {x:1, y:3};
	let y = Point {x: 2.921, y: 2.11};
	let s = Point::new(234, 111);
	println!("{}", s.x);
	println!("{}", s.y);
	println!("{}", s.isInt());
	println!("{}", y.isInt());
}