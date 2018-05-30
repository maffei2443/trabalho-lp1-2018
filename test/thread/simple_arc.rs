use std::sync::{Arc, Mutex};
use std::thread;

struct This {
    x: i32,
    y: i32
}

impl This {
    fn new(x: i32, y: i32) -> This {
        This {
            x,
            y
        }
    }
}

fn addX(s: &mut This, i: i32) {
    s.x += i;
    s.y += i;
}

fn main() {
    let mut re = This::new(10, 20);
    println!("{} {}", re.x, re.y);
    addX(&mut re, 198);
    let arc = Arc::new( Mutex::new(re) );

    println!("{} {}", arc.x, arc.y);
    // println!("{} {}", re.x, re.y);
}