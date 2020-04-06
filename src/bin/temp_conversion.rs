use std::io;

fn main() {

    println!("Enter fahrenheit to convert to celsius.");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line");
    let number: f32 = buffer.trim().parse().expect("Please type a number!");
    println!("{} is {} degrees celsius", number, get_celsius(number));

}

fn get_celsius(i: f32) -> f32{

    (i - 32.0) * (5.0/9.0)
}