
fn main(){

//    let mut v : Vec<u32> = Vec::new();
////
////    v.push(1);
////    v.push(2);
////    println!("{:?}", v);
////    v.pop();
////    println!("{:?}", v);
////
////    let s3 = String::from("test2");
////    let s4: Vec<char> = s3.chars().collect();
////    println!("{:?}",s4);
////
////
////
////
////    let vec = vec![0,1,2,3,4];
////    vec.iter().map(|x| *x+1).filter(|x| *x > 1).for_each(|x| print!("{}", x));
////    println!("{:?}",vec);

//    test_iter_mut();
//    test_fold();

}

fn test_iter_mut(){
    let mut v = vec![0,1,2,3,4];
    for i in v.iter_mut(){
        *i += 1;
    }
    println!("{:?}",v);
}

fn test_fold(){
    let v = vec![1,2,3,4];
    let sum_of_double = v.iter().fold(0, |acc, x| acc + (x * 2));
    println!("{}",sum_of_double);




}