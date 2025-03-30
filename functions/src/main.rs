fn test_function(i: u32) {
    println!("This is a test function. Printed integer: {}", i);

    let y = {
        let x = i;
        x + 1
    };
    println!("The value of y is: {}", y);
}

fn five() -> u32 {
    5
}

fn main() {
    println!("Hello, world!");
    test_function(42);
    test_function(five());
    
    for i in (1..=4).rev() {
        println!("The value of i is: {}", i);
    }
}