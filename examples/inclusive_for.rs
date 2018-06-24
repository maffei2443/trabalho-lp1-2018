fn main() {
  for i in 1..13 {
    match i {
      0...5 => println!("{} is between 0 and 5, exlcusive", i),
//      5 => println!("i is {}", i),
      6..=9 => println!("inclusive range: [6,9]: {}", i),
      -1..100 => println!("out of bounds! i is {}", i),
    }
  }
}
