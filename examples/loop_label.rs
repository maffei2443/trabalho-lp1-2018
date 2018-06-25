#![allow(unused_variables)]
fn main() {
	let (mut a, mut b) = (1, 1);
	let result = 'lup: loop {
		loop {
		    if b > 10 {
		        break 'lup b;
		    }
		    let c = a + b;
		    a = b;
		    b = c;
		};
		break 'lup -2;  // nunca será atingido
		break 99;
	};
	println!("{:?}", result); 
	assert_eq!(result, 13);
}


// #![allow(unused_variables)]
// fn main() {
// 	loop {
// 	};
// }