use std::io;

fn convert_to_celcius(n: i32) -> i32 {
    (n - 32) * 5/9
}

fn convert_to_fahrenheit(n: i32) -> i32 {
    (n * 9/5) + 32
}

fn main() {
    let mut option = String::new();
    let mut value = String::new();

    println!("Hi, welcome, this program converts degrees celcius to fahrenheit or vice versa.");
    println!("Choose an operation:");
    println!("1- Convert celsius to fahrenheit\n2- Convert fahrenheit to celsius");

    io::stdin().read_line(&mut option).expect("failed to read input");
    
    if option == "1\n" {
        println!("Enter a value: ");

        io::stdin().read_line(&mut value).expect("failed to read input");

        let value: i32 = value.trim().parse().expect("Please type a number!");

        println!("{}°F converted to Celsius are {}°C", value, convert_to_celcius(value));
    } else if option == "2\n" {
        println!("Enter a value: ");

        io::stdin().read_line(&mut value).expect("failed to read input");

        let value: i32 = value.trim().parse().expect("Please type a number!");

        println!("{}°C converted to fahrenheit are {}°F", value, convert_to_fahrenheit(value));
    }
}
