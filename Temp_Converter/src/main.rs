
use std::io;

enum Unit {
    Celsius,
    Fahrenheit,
}

struct Temperature {
    value: f64,
    unit: Unit,
}

impl Temperature {
    fn convert(&self) -> f64 {
        match self.unit {
            Unit::Celsius => {
                (self.value * 9.0 / 5.0) + 32.0
            }
            Unit::Fahrenheit => {
                (self.value - 32.0) * 5.0 / 9.0
            }
        }
    }
}

fn get_input() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}

fn main() {
    loop {
        println!("\n=== TEMPERATURE CONVERTER ===");
        println!("1. Celsius to Fahrenheit");
        println!("2. Fahrenheit to Celsius");
        println!("3. Exit");

        println!("Choose an option (1-3):");

        let choice = get_input();

        let unit = match choice.as_str() {
            "1" => Unit::Celsius,
            "2" => Unit::Fahrenheit,
            "3" => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid option. Try again.");
                continue;
            }
        };

        println!("Enter the temperature:");

        let input = get_input();

        let value: f64 = match input.parse::<f64>() {
            Ok(number) if number.is_finite() => number,
            _ => {
                println!("Invalid temperature. Enter a number.");
                continue;
            }
        };

        let temperature = Temperature {
            value,
            unit,
        };

        let result = temperature.convert();

        match temperature.unit {
            Unit::Celsius => {
                println!(
                    "{:.2} °C = {:.2} °F",
                    temperature.value,
                    result
                );
            }
            Unit::Fahrenheit => {
                println!(
                    "{:.2} °F = {:.2} °C",
                    temperature.value,
                    result
                );
            }
        }
    }
}
