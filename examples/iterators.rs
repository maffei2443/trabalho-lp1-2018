
fn main() {
	let v1 = vec![1, 2, 3];

	let v1_iter = v1.iter();
	let vsum_iter = v1_iter.map(|x| x + 1);

	for val in vsum_iter.map(|x| x+10) {
	    println!("Got: {}", val);
	}
	// consumiu vsum_iter
	let v1_iter = v1.iter().map(|x| 2*x + 18);
	println!("{:?}", v1_iter);
	
	let v1_iter = v1.iter().map(|x| 2*x + 18);

	let soma:i32 = v1_iter.sum();  // anotação de tipo requerida nesse caso
	println!("{:?}", soma);

	let v1_p10: Vec<i32> = v1.iter()
							.map(|x| 2*x + 18)
							.filter(|x| x % 3 == 0)
							.collect();  // neste caso, "coleta" os itens iterados e monta um Vector com eles.
	// let v1_p10 = v1.iter().map(|x| 2*x + 18).filter(|x| x % 3 == 0).collect();

	println!("{:?}", v1_p10);
}