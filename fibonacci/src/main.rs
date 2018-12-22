fn main() {
    let n = 10;
    println!("{}th Fibonacci number: {}", n, fibonacci(n))
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        x => fibonacci(x - 1) + fibonacci(x - 2)
    }
}
