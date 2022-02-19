use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_result = simulated_expensive_calculation(intensity);

    if intensity < 25 {
        println!("Today, Do {} pushups!", expensive_result);
        println!("Then, Do {} situps", expensive_result);
    } else {
        if random_number == 3 {
            // if we go here, we don't need to wait for expensive_result funtion, but we still wait
            println!("Take a break today! Remember to stay hydrated.");
        } else {
            println!("Today, run for {} minutes", expensive_result)
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}
