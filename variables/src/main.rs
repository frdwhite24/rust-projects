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

    // Example of ownership for data stored on the heap
    let a_string = String::from("hello");
    let mut another_string = a_string.clone();
    another_string.push_str(" world");
    println!("{}", a_string);
    println!("{}", another_string);

    // Mutable references
    let mut a = String::from("hello");
    add_fullstop(&mut a);
    add_fullstop(&mut a);
    println!("{}", a);

    // Structs
    struct Address {
        building_number: u32,
        first_line: String,
        second_line: String,
        town: String,
        postcode: String,
    }

    let building_number = 32;
    let mut my_address = Address {
        building_number, // field init shorthand
        first_line: String::from("Main Street"),
        second_line: String::from("Big Apartments"),
        town: String::from("Big city"),
        postcode: String::from("YU3 7DN"),
    };

    my_address.building_number = 24;
    println!("{}", my_address.building_number);
    println!("{}", my_address.first_line);
    println!("{}", my_address.second_line);
    println!("{}", my_address.town);
    println!("{}", my_address.postcode);
}

fn add_fullstop(value: &mut String) {
    value.push_str(".")
}
