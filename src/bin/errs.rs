use std::io;
use std::fs;


fn main(){

    let username = read_username_from_file("user.txt");
    match username{
        Ok(user) => println!("{}", user),
        Err(e) => println!("Error reading username from file!")
    };

}

fn read_username_from_file(filename : &str) -> Result<String, io::Error>{
    fs::read_to_string(filename)
}