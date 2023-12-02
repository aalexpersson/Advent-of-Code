use std::fs::File;
use std::io::{self, prelude::*, BufReader};


fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut value;
    let mut sum = 0;

    for line in reader.lines() {
        value = line.unwrap();
        sum += find_sum(value);
    }

    println!("{}",sum);

    Ok(())
    
}

fn find_sum(line: String) -> i32{
    let values = line.chars();
    let mut numbers = Vec::new();
    let mut complete = String::new();

    for c in values {
        if c.is_digit(10) {
            let number = (c.to_string()).parse::<i32>().unwrap();
            numbers.push(number);
        } 
    }    
    complete.push_str(&numbers[0].to_string());
    complete.push_str(&numbers[numbers.len() - 1].to_string());
    return complete.parse::<i32>().unwrap();
}
