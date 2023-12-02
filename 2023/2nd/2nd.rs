use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        //println!("{:?}", line.unwrap());
        let value = line.unwrap();
        find_sum(value);
    }

    Ok(())
    
}
