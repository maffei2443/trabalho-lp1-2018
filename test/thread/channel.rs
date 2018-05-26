use std::thread;
use std::sync::mpsc;  // mpsc: multiple producer, single consumer
use std::sync::{Arc, Mutex};

// brincando com Mutex!
// use std::sync::Mutex as Mut; 



fn main() {
    let (tx, rx) = mpsc::channel();  // tx = transmitter; rx = receiver
    thread::spawn(move || {
        let val = String::from("hi");
        for _i in 1..100 {
        	thread::sleep( std::time::Duration::from_millis(1) );
        }
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();  // Bloqueia a thread até receber algo; tipo uma variável de condição
    // let try_received = rx.try_recv().unwrap();  // Ouve o canal, mas não bloqueia a thread; tipo uma variável de condição
    println!("Got: {}", received);
}