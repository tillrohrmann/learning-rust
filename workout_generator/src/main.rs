use std::thread;
use std::time::Duration;
use std::collections::HashMap;

fn main() {
    let intensity = 7;
    let random_number = 42;

    generate_workout(intensity, random_number);
    generate_workout(intensity + 1, random_number + 1);
}

struct Cacher<T> where T: Fn(u32) -> u32 {
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, intensity: u32) -> u32 {
        let function = &self.calculation;
        *(self.value.entry(intensity)
            .or_insert_with(|| function(intensity)))
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
