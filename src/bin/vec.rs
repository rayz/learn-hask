
fn main(){

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