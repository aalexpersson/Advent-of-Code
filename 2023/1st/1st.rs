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
    let mut numbers = Vec::new();
    let mut complete = String::new();

    //let second_element = &list[1];

    for c in values {
        if c.is_digit(10) {
            //println!("{}", c);
            let number = (c.to_string()).parse::<i32>().unwrap();
            numbers.push(number);
        } 
    }    
    complete.push_str(&numbers[0].to_string());
    complete.push_str(&numbers[numbers.len() - 1].to_string());
    println!("{}", complete)
}
