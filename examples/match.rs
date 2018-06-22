
#![allow(unused_variables)]
fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    println!("{:?}",Some(9));
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    let ten = plus_one(9);
    println!("{:?}", five);
    println!("{:?}", six);
    println!("{:?}", none);
    println!("{:?}", );
}
