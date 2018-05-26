#[allow(unused_imports)]
#[allow(dead_code)]

use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc::channel;

fn foo(mu: Arc<Mutex<i32> >) {
	thread::sleep(std::time::Duration::from_millis(100));  // espera 1 segundo
	let mut data = mu.lock().unwrap();
	println!("{}", data);
	*data += 1
}

fn main() {
	let n = 90;
	let data = Arc::new(Mutex::new(n));

	// let (tx, rx) = channel();
	#[allow(unused_assignments)]
	let mut r = thread::spawn(move || {});
	#[allow(unused_assignments)]
	let mut s = thread::spawn(move || {});
	#[allow(unused_assignments)]
	let mut t = thread::spawn(move || {});
	{	
		let dado = data.clone();
		r = thread::spawn(move || foo(dado.clone()) );
	}
	{	
		let dado = data.clone();
		s = thread::spawn(move || foo(dado.clone()) );
	}
	{	
		let dado = data.clone();
		t = thread::spawn(move || foo(dado.clone()) );
	}
	r.join().unwrap();
	s.join().unwrap();
	t.join().unwrap();
	println!("Terminaram as threads");
}