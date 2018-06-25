fn main() {
	let v1 = vec![1, 2, 3];
	let v1_p10: Vec<i32> = v1.iter()
							.map(|x| 2*x + 18)
							.filter(|x| x % 3 == 0)
							.collect();  // neste caso, "coleta" os itens iterados e monta um Vector com eles.
	// let v1_p10 = v1.iter().map(|x| 2*x + 18).filter(|x| x % 3 == 0).collect();

	println!("{:?}", v1_p10);
}