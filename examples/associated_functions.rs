#[derive(Debug)]
struct Triangle {
    l1: i32,
    l2: i32,
    l3: i32,
}

impl Triangle {
    fn equilater(size: i32) -> Triangle {
        Triangle{l1: size, l2: size, l3: size}
    }
}

fn main() {
    println!("{:?}", Triangle::equilater(100));
}