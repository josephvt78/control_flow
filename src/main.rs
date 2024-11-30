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
}
