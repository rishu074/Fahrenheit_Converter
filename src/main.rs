use std::io;


fn c_to_f(c: f64) -> f64 {
    c * 9.0/5.0 + 32.0
}

fn f_to_c(f: f64) -> f64 {
    (f - 32.0) * 5.0/9.0
}


fn main() {
    println!("Starting Converter.... :)\n\n");

    println!("Enter the temperature.");
    let mut temperature = String::new();

    io::stdin().read_line(&mut temperature).expect("Failed to read temperature!");
    let temperature: f64 = temperature.trim().parse().expect("Please enter a number!");


    println!("Enter 'F' to convert Fahrenheit to Celsius or Enter 'C' to convert Celsius to Fahrenheit.");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read the input");

    let input = input.trim();
    match input {
        "F" => println!("l"),
        "C" => println!("2"),
        _ => {
            println!("Else");
            return
        }
    }

    println!("EX")
}
