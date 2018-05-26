fn main(){
    println!("CANT Add string literals.. :(");
    println!("Must be std::Str::String + &str");
    println!("{}", "Hi".to_string() + ", boy!");
    println!("{}", "Hi".to_owned() + ", boy!");

    println!("\nAnother way to do that is using 'format', which DOESN'T take any ownership");
    println!("like: let s = format!(\"{{}}-{{}}-{{}}\", s1, s2, s3);");
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{:?}\n", s);
}