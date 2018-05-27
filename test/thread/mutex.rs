use std::sync::{Mutex, Arc};
use std::thread;

#[allow(non_snake_case)]
fn modLock(x: &mut i32) {
    *x *= 5;
    println!("Mult 5");
}


fn top(x: Arc<Mutex<i32>>) {
    let mut y = x.lock().unwrap();
    *y += 10;
    println!("topou! {}", *y);
    modLock(&mut *y);
}

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // let mut num = counter.lock().unwrap();
            top(counter);
            // *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}