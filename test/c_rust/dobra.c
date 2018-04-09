// This code was taken from:
// https://blog.rust-lang.org/2015/04/24/Rust-Once-Run-Everywhere.html

int double_input(int input) {
    return input * 2;
}
// To call this from Rust, you might write a program like this:

// extern crate libc;

// extern {
//     fn double_input(input: libc::c_int) -> libc::c_int;
// }

// fn main() {
//     let input = 4;
//     let output = unsafe { double_input(input) };
//     println!("{} * 2 = {}", input, output);
// }