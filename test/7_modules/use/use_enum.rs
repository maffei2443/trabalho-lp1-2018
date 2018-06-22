// https://doc.rust-lang.org/book/second-edition/ch07-03-importing-names-with-use.html
enum TrafficLight {
    Red,
    Yellow,
    Green,
}
// Separate by commas when wants to bring more than one iten from a namespace into scope !!!
use TrafficLight::{Red, Yellow};

fn main() {
    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;
}