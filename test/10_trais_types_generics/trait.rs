fn largesti32(list: &[i32]) -> i32 {
	let mut max = list[0];

	for &item in list.iter() {
		if item > max {
			max = item;
		}
	}
	max
}

// função genérica sobre um tipo T.
fn largest<T>(list: &[T]) -> T {
	let mut max = list[0];

	for &item in list.iter() {
		if item > max {
			max = item;
		}
	}
	max
}


fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}