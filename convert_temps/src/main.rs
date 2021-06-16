use std::io;

fn main() {
    let direction = get_convert_direction();
    let value = get_value_to_convert();
    let result = convert_value(direction, value);
    println!("{}", result)
}

fn get_user_input(message: &String) -> String {
    println!("{}", message);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}

fn get_convert_direction() -> u32 {
    let message = String::from("Are you converting from: [1] F to C, or [2] C to F?");
    let answer = get_user_input(&message);
    let answer: u32 = answer.trim().parse().expect("Please input a number");
    return answer;
}

fn get_value_to_convert() -> f64 {
    let message = String::from("Enter value to be converted:");
    let value = get_user_input(&message);
    let value: f64 = value.trim().parse().expect("Please input a number");
    return value;
}

fn convert_value(direction: u32, value: f64) -> f64 {
    if direction == 1 {
        fah_to_cel(value)
    } else {
        cel_to_fah(value)
    }
}

fn cel_to_fah(value: f64) -> f64 {
    (value * 1.8) + 32.0
}

fn fah_to_cel(value: f64) -> f64 {
    (value - 32.0) / 1.8
}
