struct Point<T> {
	x: T,
	y: T
}

impl<T> Point<T> {
	fn new(x: T, y: T) -> Point<T> {
		Point {	x, y }
	}
}
use std::fmt;

impl<T: fmt::Display> fmt::Display for Point<T> {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		write!(formatter, "x: {}, y: {}", self.x, self.y)  // coloca dados no buffer
	}
}

static mut boxa : Vec<i32> = vec![1,2,3];

fn nada(v: &mut Vec<i32>) {
	println!("{:?}", v);
}


fn main() {
	// let ponto = Point::new(2.33,  4.12);
	// println!("{}", ponto);
	// let v = [1,2,3];
	// let it = v.iter();

	// for (l, r) in (it, it) {
	// 	println!("{}", Point::new(r,l) );
	// }
	
	std::thread::spawn(|| nada(&mut vet));
	for i in vet.iter() {
		println!("{:?}", i);
	}
}