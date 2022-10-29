use std::io;

fn main() {
    loop {
        println!("Temperature you want to convert from Fahrenheit to Celsius:");

        let mut temperature = String::new();

        let stdin = io::stdin();

        stdin
            .read_line(&mut temperature)
            .expect("Can not read the input");

        let temperature:&str = temperature.trim();

        let mut num:i32 = temperature.parse().unwrap();

        num = (num - 32) * 5 / 9;

        println!("{} Fahrenheit is equal to {} Celsius", temperature, num);
    }
}
