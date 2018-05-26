fn main() {
    let x = 5;
    let y = &x;

    println!("{:?}",  assert_eq!(5, x));
    println!("{:?}",  assert_eq!(5, *y));
    // println!("{:?}",  assert_eq!(y, *y));
    println!("{:?}",  y);
    println!("{:?}",  *y);
    // y == *y;
}