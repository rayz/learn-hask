#[derive(Debug)]
struct Food {
    name: String,
    cals: u32,
}

impl Food {
    fn is_healthy(&self) -> bool {
        println!("cals is : {}", self.cals);
        self.cals < 200
    }
}

fn main() {
    let apple = Food {
        name: "apple".to_string(),
        cals: 50,
    };
    println!("{:#?} is healthy? {}", apple, apple.is_healthy());
}
