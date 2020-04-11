fn main(){
    let x = Some(3);
    match x {
        Some(3) => println!("3!"),
        _ => println!("not 3!")
    };
}