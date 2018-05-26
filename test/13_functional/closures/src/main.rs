use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn  add_one_v1   (x: u32) -> u32 { x + 1 }

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    let x = 32;
	let add_one_v2 = |x: u32| -> u32 { x + 1 };
	let add_one_v3 = |x|             { x + 1 };
	let add = add_one_v3(22);						// define o tipo do parâmetro que a closure add_one_v3 vai receber!
	let add_one_v4 = |x: i32|               x + 1  ;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
	let example_closure = |x| x;

	let s = example_closure(String::from("hello"));  // Após chamar pela primeira vez a closure, "TRAVA" o tipo inferido para x!
	let n = example_closure(5);
}

fn generate_workout2(intensity: u32, random_number: u32) {
    let expensive_result =
        simulated_expensive_calculation(intensity);

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result
        );
        println!(
            "Next, do {} situps!",
            expensive_result
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result
            );
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_closure(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_closure(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure(intensity)
            );
        }
    }
}