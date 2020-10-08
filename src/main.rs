use std::io;

const OFFSET: f64 = 32.0;
const WEIGHT_FACTOR: f64 = 5.0 / 9.0;

fn main() {
    println!("Do you want convert  Celsius or Fahrenheit? Input C or F !");

    let mut convert_type = String::new();

    io::stdin()
        .read_line(&mut convert_type)
        .expect("Error input covert type");

    let t = String::from(convert_type);

    println!("You want to convert from {}", t);
    println!("What temperature would you like to convert ?");

    let mut temp = String::new();

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to insert temp value");

    let temp: f64 = temp.trim().parse().expect("Cannot parse input");

    let mut result = 0.0;

    match t.as_str() {
        "C\n" => result = cel_to_far(temp),
        "F\n" => result = far_to_cel(temp),
        _ => println!("t = {:?}", t),
    };

    println!("Input {}", temp);
    println!("Result {}॰ ", result)
}

fn cel_to_far(temp: f64) -> f64 {
    temp / WEIGHT_FACTOR + OFFSET
}

fn far_to_cel(temp: f64) -> f64 {
    (temp - OFFSET) * WEIGHT_FACTOR
}
