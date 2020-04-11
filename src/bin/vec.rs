
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




    let vec = vec![0,1,2,3,4];
    vec.iter().map(|x| *x+1).filter(|x| *x > 1).for_each(|x| print!("{}", x));
    println!("{:?}",vec);


}
