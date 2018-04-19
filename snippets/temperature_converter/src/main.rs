use std::io;

fn main() {
    println!("Enter a temperature in Celsius, I will convert it to Fahrenheit");

    let mut temp_celsius = String::new();
    io::stdin().read_line(&mut temp_celsius)
        .expect("Failed to read line");

    let temp_celsius: f64 = temp_celsius.trim().parse()
        .expect("Failed to convert number, please enter a floating point number");
    //let temp_celsius: f64 = match temp_celsius.trim().parse() {
    //    Ok(num) => num,
    //    Err(_)  => continue,
    //};

    let temp_fahrenheit = (temp_celsius * 9.0 / 5.0) + 32.0;

    println!("Temperature in Celsius = {}, in Fahrenheit = {}"
             , temp_celsius, temp_fahrenheit);

}

