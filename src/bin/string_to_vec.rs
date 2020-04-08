fn main(){
    let s = String::from("hello world!");
    let x: Vec<&str> = s.split(" ").collect();

    println!("{:?}",x);
}