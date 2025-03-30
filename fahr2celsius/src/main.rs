use std::io;

fn fahr2celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius2fahr(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}


fn main() {
    println!("Enter 'f' to convert Fahrenheit to Celsius or 'c' to convert Celsius to Fahrenheit:");
    let mut mode = String::new();
    io::stdin()
        .read_line(&mut mode)
        .expect("Failed to read line");

    let mode = mode.trim().to_lowercase();
    if mode == "f" {
        let mut fahrenheit = String::new();
        println!("Enter temperature in Fahrenheit:");
        io::stdin()
            .read_line(&mut fahrenheit)
            .expect("Failed to read line");
        let fahrenheit: f64 = fahrenheit.trim().parse().expect("Please enter a number");
        let celsius = fahr2celsius(fahrenheit);
        println!("{}°F is {:.2}°C", fahrenheit, celsius);
    } else if mode == "c" {
        let mut celsius = String::new();
        println!("Enter temperature in Celsius:");
        io::stdin()
            .read_line(&mut celsius)
            .expect("Failed to read line");
        let celsius: f64 = celsius.trim().parse().expect("Please enter a number");
        let fahrenheit = celsius2fahr(celsius);
        println!("{}°C is {:.2}°F", celsius, fahrenheit);
    } else {
        println!("Invalid mode. Please enter 'f' or 'c'.");
    }
}
