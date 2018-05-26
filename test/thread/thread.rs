use std::thread;
use std::time::Duration;

fn main() {
    let wait = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    // Bloqueia a thread atual até que a 'thread wait' termine
    wait.join().unwrap();
}