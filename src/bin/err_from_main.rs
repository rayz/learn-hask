use std::fs::File;
use std::io;

fn main() -> std::io::Result<()> {
    let mut file = File::open("foo.txt")?;

    Ok(())
}
