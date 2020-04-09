

fn main(){

    let mut s1 = String::from("hello");

    /*

        passing in arg will move the ownership of the object
        to avoid, pass in the reference of the object instead

    */
    let len = calculate_length(&s1);

    println!("{}'s length is {}", s1, len);

    change(&mut s1);

    println!("after change, s1 is : {}",s1);


    let mut s2 = String::from("test");
    let r1 = &mut s2;

    let mut s3 = String::from("test2");
    let r2 = &mut s3;
    println!("r1 is {} , r2 is {}", r1, r2);

//    let r3 = &mut s3;
//    cant have two mutable references at sametime
//        (this is to avoid data races)



}

fn calculate_length(some_string: &String) -> usize{

    some_string.len()

}

fn change(some_string: &mut String){
    some_string.push_str(" world!");
}