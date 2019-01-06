use std::thread;
use std::time::Duration;

fn main() {
    let intensity = 7;
    let random_number = 42;

    generate_workout(intensity, random_number);
}

struct Cacher<T> where T: Fn(u32) -> u32 {
    calculation: T,
    value: Option<u32>
}

impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None
        }
    }

    fn value(&mut self, intensity: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let value = (self.calculation)(intensity);
                self.value = Some(value);
                value
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_calculation = Cacher::new(|num| {
        println!("Executing slow calculation.");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Do {} push-ups.", expensive_calculation.value(intensity));
        println!("Do {} sit-ups.", expensive_calculation.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break.");
        } else {
            println!("Go for a run for {} minutes.", expensive_calculation.value(intensity));
        }
    }
}
