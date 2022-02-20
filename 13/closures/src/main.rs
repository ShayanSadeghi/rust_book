use std::thread;
use std::time::Duration;

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculation slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, Do {} pushups!", expensive_result.value(intensity));
        println!("Then, Do {} situps", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            // now we don't call closure because we don't need that here
            println!("Take a break today! Remember to stay hydrated.");
        } else {
            println!(
                "Today, run for {} minutes",
                expensive_result.value(intensity)
            )
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    let x = 4;
    let equal_to_x = |z| z == x; // simple closure. it has access to x, which we won't have access to if we use a function

    let y = 4;

    assert!(equal_to_x(y));
}
