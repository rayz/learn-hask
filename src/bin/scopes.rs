fn main() {
    let x = "hello";
    {
        let y = "bye";

        println!("{}, {}", x, y);
    }
    println!("{}", x);
    //    ERROR
    //    x still in scope but y is not
    //    println!("{}",y);
}
