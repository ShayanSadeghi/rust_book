use std::thread;
use std::time::Duration;

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("calculation slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        let expensive_res = expensive_closure(intensity);
        println!("Today, Do {} pushups!", expensive_res);
        println!("Then, Do {} situps", expensive_res);
    } else {
        if random_number == 3 {
            // now we don't call closure because we don't need that here
            println!("Take a break today! Remember to stay hydrated.");
        } else {
            println!("Today, run for {} minutes", expensive_closure(intensity))
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}
