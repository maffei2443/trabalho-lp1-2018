fn main() {
	fn  add_one_v1   (x: u32) -> u32 { x + 1 }
	let add_one_v2 = |x: u32| -> u32 { x + 1 };
	let add_one_v3 = |x|             { x + 1 };
	let add_one_v4 = |x|               x + 1  ;
	
	println!("add_one_v1(0) == {:?}", add_one_v1(0));
	println!("add_one_v2(22) == {:?}", add_one_v2(22));
	println!("add_one_v3(123) == {:?}", add_one_v3(123));
	println!("add_one_v4(2434) == {:?}", add_one_v4(2434));
}