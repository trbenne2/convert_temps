use std::io;

const WATER_FREEZING_POINT_F: f64 = 32.0;
fn main() {
    println!("Enter in c if you want to convert from Celsius to Fahrenheit or f if you want to convert from Fahrenheit to Celsius: ");

    let mut conversion_type = String::new();

    io::stdin()
        .read_line(&mut conversion_type)
        .expect("Failed to read line");

    if conversion_type.trim() == "c" {
        println!("Enter the temperature in Celsius: ");
        let mut celsius = String::new();
        io::stdin()
            .read_line(&mut celsius)
            .expect("Failed to read line");
        let celsius: f64 = celsius.trim().parse().expect("Please type a number!");
        let fahrenheit = (celsius * 9.0 / 5.0) + WATER_FREEZING_POINT_F;
        println!("{}°C is equal to {}°F", celsius, fahrenheit);
    } else if conversion_type.trim() == "f" {
        println!("Enter the temperature in Fahrenheit: ");
        let mut fahrenheit = String::new();
        io::stdin()
            .read_line(&mut fahrenheit)
            .expect("Failed to read line");
        let fahrenheit: f64 = fahrenheit.trim().parse().expect("Please type a number!");
        let celsius = (fahrenheit - WATER_FREEZING_POINT_F) * 5.0 / 9.0;
        println!("{}°F is equal to {}°C", fahrenheit, celsius);
    } else {
        println!("Invalid input. Please enter 'c' or 'f'.");
    }
}
