use std::io;

fn main() {
    let message = String::from("Which nth fibonacci number would you like to calculate?");
    loop {
        let position = get_user_input(&message);
        let position: i32 = position.trim().parse().expect("Please input a number");
        let result = get_fibonacci_number(position);
        let result = result as i64;
        println!("Position {}: {}", position, result);
    }
}

fn get_user_input(message: &String) -> String {
    println!("{}", message);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}

fn get_fibonacci_number(position: i32) -> f64 {
    const GOLDEN_RATIO: f64 = 1.618034;
    (GOLDEN_RATIO.powi(position) - (1.0f64 - GOLDEN_RATIO).powi(position)) / 5.0f64.sqrt()
}
