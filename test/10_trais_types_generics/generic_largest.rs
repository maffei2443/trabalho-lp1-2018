// fn bigger<T>(lis: &[T]) -> T 
// where T: std::cmp::PartialOrd{ 
//     let mut big = lis[0];
//     for &i in lis.iter() {
//         if i > big {
//             big = i;
//         }
//     }
//     big
// }

fn largest<T>(list: &[T]) -> T where T: PartialOrd + Copy{
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}


fn main() {
    let lchar = vec!['a', 't', 't', 'e'];
    println!("{}", largest(&lchar));
    let lchar = vec!["0", "9", "-1", "1"];
    
    println!("{}", largest(&lchar));
}