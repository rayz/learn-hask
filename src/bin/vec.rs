
fn main(){

    let mut v : Vec<u32> = Vec::new();
    
    v.push(1);
    v.push(2);
    println!("{:?}", v);
    v.pop();
    v.pop();
    println!("{:?}", v);
    v.pop();
    v.pop();
    v.pop();
    v.pop();
    v.pop();


    let s3 = String::from("test2");
    let s4: Vec<char> = s3.chars().collect();
    println!("{:?}",s4);


}
