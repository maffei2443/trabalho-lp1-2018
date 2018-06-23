fn main() {
	let v0: Vec<i32> = Vec::new();
	let v1 = vec![2, 1, 3];

	let thirdRisk: &i32 = &v0[2];
	let thirdSafe: Option<i32> = v0.get(2);
}