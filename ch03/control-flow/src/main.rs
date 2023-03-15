fn main() {
    let number = 7;

    // Always need to have boolean in condition.
    // Can't do if number { ... }, because int != bool
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other than zero");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3 or 2");
    }

    let condition = true;
    let othernumber = if condition { 5 } else { 6 };

    println!("The value of number is: {othernumber}");
}
