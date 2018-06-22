fn main(){
  let x = 9;
  println!("x imutavel == {}", x);
  let mut x = x;
  println!("x agora eh mutavel, mas vale {}", x);
  x = 1000;
  println!("Valor atual de x: {}", x);
}
