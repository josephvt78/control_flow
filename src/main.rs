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
}
