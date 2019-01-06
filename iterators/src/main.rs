fn main() {
    let counter = Counter::new();

    for i in counter {
        println!("Value {}.", i);
    }

    let a = Counter::new();
    let b = Counter::new();

    let result: u32 = a.zip(b.skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();

    println!("Iterator result {}.", result);
}

struct Counter {
    counter: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter {
            counter: 0,
        }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        self.counter += 1;

        if self.counter < 6 {
            Some(self.counter)
        } else {
            None
        }
    }
}
