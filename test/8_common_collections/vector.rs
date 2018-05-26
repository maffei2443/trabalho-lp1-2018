fn main() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    for i in &v{
        println!("{:?}", i);
    }
    let v: Vec<i32> = Vec::new();

    let v2: std::vec::Vec<i32> = std::vec::Vec::new();

    let v3 = vec![1,2,3];
    println!("v == v2 ? {}", v == v2);
    println!("{:?}", v3);

    let third: &i32 = &v3[2];
    let safe_third: Option<&i32> = v3.get(2);
    println!("third: {:?}", third);
    println!("safe_third: {:?}", safe_third);

    println!("Not gonna pannic: {:?}", v3.get(100));

    println!("Gonna panic !!!"); 
    v3[100];
}