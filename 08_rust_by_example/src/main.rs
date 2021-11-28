use std::fmt;

fn main() {
    hello_world()
}

fn hello_world() {
    println!("Hello {name}!", name = "Fred");
    println!("I'm a Rustacean!");

    // Checks number of args being entered
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // Implementing the Display trait for our custom tuple struct
    struct Structure(i32);
    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    println!("This struct `{}` won't print...", Structure(3));

    // Control the decimal places
    let pi = 3.141592;
    println!("Pi is roughly {:.3}", pi)
}
