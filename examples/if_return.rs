fn main() {
	let mut v = vec![1, 2, 3];
	v.pop();          // Ignore the element returned from pop
	println!("{}", if v.is_empty() { v.push(5);"Was empty"}
					 else { v.remove(0); "Wasn't empty"}                 // Semicolon can be omitted.
			);
	println!("{}", if false {false}else {true});
}