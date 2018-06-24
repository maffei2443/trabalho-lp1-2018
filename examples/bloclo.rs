fn main(){
  let x = 32;
  {
	let x = 42;
	println!("{}", super::x); 
  }
}
