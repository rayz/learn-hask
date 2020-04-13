fn main() {
    //    test_iter_mut();
    //    test_fold();
    test_enum();
}

fn test_iter_mut() {
    let mut v = vec![0, 1, 2, 3, 4];
    for i in v.iter_mut() {
        *i += 1;
    }
    println!("{:?}", v);
}

fn test_fold() {
    let v = vec![1, 2, 3, 4];
    for &i in v.iter(){
        let y = i;
    }
    let sum_of_double = v.iter().fold(0, |acc, x| acc + (x * 2));
    println!("{}", sum_of_double);
}

fn test_enum() {
    #[derive(Debug)]
    enum Multi {
        Number(i32),
        String(String),
    }
    let mut v: Vec<Multi> = Vec::new();
    v.push(Multi::Number(100));
    v.push(Multi::String("hundred".to_string()));

    println!("{:?}", v);

}
