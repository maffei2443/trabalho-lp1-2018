enum Int {
	i(i64)
}

enum Float {
	f(f64)
}

struct Gambs {
	Int: i64,
	Float: f64,
}

use std::boxed::*;

fn main() {
	let x = Gambs{Int: 900, Float: 0 as f64};
	let y = match x {
			Gambs.Int => Box::new(algo),
			Gambs.Float => Box::new(fruta),
		};
	println!("{:?}", y);
}