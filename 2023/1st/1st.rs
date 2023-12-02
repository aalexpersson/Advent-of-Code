use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut value;
    let mut sum = 0;

    for line in reader.lines() {
        value = line.unwrap();
        sum += find_sum(value.clone());
        find_sum(value);
    }

    println!("{}",sum);

    Ok(())
    
}

fn find_sum(line: String) -> i32{
   // let mut values = line.chars();
    let mut numbers = Vec::new();
    let mut substring_vec = Vec::new();
    let mut replacement;
    let mut complete = String::new();

    let word_digit = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);
   
    for i in 0..line.len() {
        for j in i..line.len() {
            let substring = &line[i..=j];

            // Check if the substring contains a key in the hashmap
            if word_digit.contains_key(substring) {
                substring_vec.push(substring);
               //println!("{:?}", substring_vec);
            }
        }
    }

    let mut new_line = line.clone();
    for entry in substring_vec{
        replacement = new_line.replace(entry, &word_digit.get(entry).unwrap().to_string());
        new_line = replacement.clone();
    }
    let values = new_line.chars();
    for c in values {
        if c.is_digit(10) {
            let number = (c.to_string()).parse::<i32>().unwrap();
            numbers.push(number);
            //println!("{}", number);
        } 
    }   
    //print!("{:?}", numbers);
    complete.push_str(&numbers[0].to_string());
    complete.push_str(&numbers[numbers.len() - 1].to_string());
    println!("{}", complete);
    return complete.parse::<i32>().unwrap();
}
