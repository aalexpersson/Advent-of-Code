use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut value;

    for line in reader.lines() {
        //println!("{}", line?);
        value = line.unwrap();
        println!("{}", value);
        find_digits(value);
    }

    Ok(())

}

fn find_digits(line: String) {
    let values = line.chars();
    for c in values {
        if c.is_digit(10) {
            println!("{}", c);
        } 
    }    
}
