fn main() {
    let converted = convert(69);
    println!("Celcius: {converted}");
}

fn convert(fahrenheit: i32) -> i32 {
    let celcius = (fahrenheit - 32) * 5/9;
    return celcius;
}
