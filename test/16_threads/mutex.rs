use std::sync::Mutex;

fn show<T: std::fmt::Display + std::ops::Deref>(x: T) {
    println!("{}", *x);
}

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}
