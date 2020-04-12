fn main() {
    let x = Some(3);
    match x {
        Some(3) => println!("3!"),
        _ => println!("not 3!"),
    };

    let v = vec![1, 2, 3];
    match v.get(4) {
        Some(n) => println!("{}", n),
        None => println!("There is no 4th elem"),
    };
}
