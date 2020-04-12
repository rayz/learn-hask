
#[derive(Debug)]
struct Point<T, U>{
    x: T,
    y: U,
}

fn main(){
    test_struct_generics();
}

fn test_struct_generics(){
    let p = Point{x: 5, y:4.3};
    println!("{:?}", p);
}