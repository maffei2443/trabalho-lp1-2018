fn main() {
	let y = 9;	let x = &y;
	{
		let y = 18;
		println!("y dentro do loop: {:?}", y);
	}
	println!("y logo apos o loop: {:?}", y);

	let y = 24;
	println!("Atual valor de y: {:?}", y);
	println!("Antigo valor de y: {:?}", x);
}