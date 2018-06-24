fn main() {
  for i in -2..=13 {
     match i  {
      0...5 => println!("{} belongs to 0...5 ", i),
//      5 => println!("i is {}", i),
      6..= 9 => println!("{} BELONGS to 6..=9", i),
//      -1..100 => println!("out of bounds! i is {}", i),
     -1...100 => println!("out of bounds! i is {}", i),
      _ => (),
    }
  }
}
