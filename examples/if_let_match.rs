fn main() {
	let x = 1 == 0;
	// let x = 1 == 1;
	let some: Option<i32> = None;	// sem Option<T>, causa erro de inferência de tipo no Some(i) = some
	let ret = if x {
		println!("x is {:?}", x);
	}
	else if let Some(i) = some {
	 	println!("some == Some({})", i);
	}
	else if let None = some {
		println!("None");
	};  // ; necessario pois termina a atribuicao da linha 5
	println!("if returned: {:?}", ret);
	let some = Some("not_none");	
	println!("match: {:?}", match some {
		None => None,
		Some(x) => Some(x),
	}.unwrap());  // gonna PANIC, if some is None
}