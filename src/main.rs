use std::io;

fn main() {
    let number = 9;

    if number < 5 {
        println!("number less than 5");
    } else if number < 10 {
        println!("number greater than 5 and less than 10");
    } else {
        println!("number greater than or equal to 10");
    }

    if number != 0 { //need to have comparison operator, can't have the condition like 'if number { ... }
        println!("number has value");
    }

    //mutliple else if statements
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    //if expression
    let condition = true;
    let number = if !condition { 5 } else { 6 };

    //let number = if condition { 5 } else { "six" }; //gives error compile time

    println!("The value of number is: {number}");

    //returning values from loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    // return; // if this presents the compiler gives warning about unreachable statement
    println!("The result is {result}");

    //loop labels to disambiguate between multiple loops
    let mut count = 0;
    'counting_loop: loop { //loop labels to start with a single quote
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_loop;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    //conditional loops with while
    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    //for loop
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..10).rev() {
        println!("{number}");
    }
    println!("lift-off!!");

    //temperature conversion
    println!("Temperature Conversion");
    println!("1. Convert Celsius to Fahrenheit");
    println!("2. Convert Fahrenheit to Celsius");
    println!("Enter your choice:");

    // Read user input for choice
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");
    let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid choice. Please enter 1 or 2.");
            return;
        }
    };

    match choice {
        1 => celsius_to_fahrenheit(),
        2 => fahrenheit_to_celsius(),
        _ => println!("Invalid choice. Please enter 1 or 2."),
    }
}

fn celsius_to_fahrenheit() {
    println!("Enter temperature in Celsius:");
    let mut celsius = String::new();
    io::stdin()
        .read_line(&mut celsius)
        .expect("Failed to read input");

    let celsius: f64 = match celsius.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid temperature. Please enter a valid number.");
            return;
        }
    };

    let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
    println!("{celsius}°C is equal to {fahrenheit:.2}°F");
}

fn fahrenheit_to_celsius() {
    println!("Enter temperature in Fahrenheit:");
    let mut fahrenheit = String::new();
    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Failed to read input");

    let fahrenheit: f64 = match fahrenheit.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid temperature. Please enter a valid number.");
            return;
        }
    };

    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
    println!("{fahrenheit}°F is equal to {celsius:.2}°C");
}
