use std::iter;

fn main() {
    //char is 4 bytes instead of 1
    let emoji = "🌀";
    let repeated: String = iter::repeat(emoji).take(5).collect();

    println!("wow rust has emojis {}", repeated);
}
