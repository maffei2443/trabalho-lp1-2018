fn main() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(&s2);
    s1.push_str(s2);
    println!("{:?}", s1);
    println!("s2 is {}", s2);
    println!("{:?}", s1);

    let mut s2 = String::from("banana");
    s2.push_str(&s1);
    println!("String.push_str(&String): {}", s2);
}