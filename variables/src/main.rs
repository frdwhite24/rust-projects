fn main() {
    // Mutating variables
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    // this results in a compile error
    // x = "hello"

    // Constants
    const DAYS_IN_YEAR: i32 = 365;
    println!("{}", DAYS_IN_YEAR);

    // Shadowing allows mutating data type
    let y = "    ";
    let y = y.len() * 4;
    println!("{}", y);
}
