fn main() {
  let x;
  if let Some(y) = Option::Some(190) {
    println!("Some val is {}", y);
    x = 19.0;
  }
  else {
    x = 9.;    
  }
  println!("{}", x);
}
