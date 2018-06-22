fn main() {
	// let var = 2123;
	let mut i = 0;
	let ret = loop {
		println!("i is {}", i);
		i += 1;
		if i > 5 {
			break 4;
		}
	};
	println!("Retorno do loop: {:?}", ret);

	let ret = while i > 0 {
		i-=1;
		println!("i is {}", i);
	};
	println!("While retorna sempre : {:?}", ret);
}