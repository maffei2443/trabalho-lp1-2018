// This rust code was taken from:
// https://blog.rust-lang.org/2015/04/24/Rust-Once-Run-Everywhere.html

extern crate libc;

extern {
    fn double_input(input: libc::c_int) -> libc::c_int;
}

fn main() {
    let input = 4;
    let output = unsafe { double_input(input) };
    println!("{} * 2 = {}", input, output);
}