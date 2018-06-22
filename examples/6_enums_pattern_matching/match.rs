enum Val {
  Real,
  Dolar,
  Euro,
  Bolivar,

}

fn coin(x: &Val) -> i32{
  match x {
	&Val :: Real => 2,
	&Val :: Dolar => 3,
	&Val :: Euro => 4,
	&Val :: Bolivar => 1,
  }
}

fn is_dolar(x: &Val) -> bool{
  match x {
	&Val :: Dolar => true,
	_ => false,
  }
}

fn main() {
  let real = Val :: Real;
  let dolar = Val :: Dolar;
  let euro = Val :: Euro;
  let bolivar = Val :: Bolivar;
  println!("{}", coin(&real));
  println!("{}", coin(&dolar));
  println!("{}", coin(&euro));
  println!("{}", coin(&bolivar));
  println!("'dolar' is dolar? {}", is_dolar(&dolar));
  println!("'real' is dolar? {}", is_dolar(&real));

}
