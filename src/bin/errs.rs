use std::io;
use std::io::Read;
use std::fs::File;


fn main(){

    let username = read_username_from_file("user.txt");
    match username{
        Ok(user) => println!("{}", user),
        Err(e) => println!("Error reading username from file!")
    };

}

fn read_username_from_file(filename : &str) -> Result<String, io::Error>{
    let mut s = String::new();
    File::open(filename)?.read_to_string(&mut s)?;
    Ok(s)
}