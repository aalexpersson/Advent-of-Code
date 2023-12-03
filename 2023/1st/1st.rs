use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;
use std::collections::BTreeMap;

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut value;
    let mut sum1 = 0;
    let mut sum2 = 0;

    for line in reader.lines() {
        value = line.unwrap();
        sum1 += find_sum1(value.clone());
        sum2 += find_sum2(value.clone());
    }

    println!("Sum 1: {}", sum1);
    println!("Sum 2: {}", sum2);

    Ok(())
}

fn find_sum1(line: String) -> u32{
    let values = line.chars();
    let mut numbers = Vec::new();
    let mut complete = String::new();
   
    for c in values{
        if c.is_digit(10) {           
            let number = (c.to_string()).parse::<u32>().unwrap();
            numbers.push(number);
        } 
    }   
    complete.push_str(&numbers[0].to_string());
    complete.push_str(&numbers[numbers.len() - 1].to_string());
    return complete.parse::<u32>().unwrap();
}

fn find_sum2(line: String) -> u32{
    let mut substring_vec = Vec::new();
    let mut indices = BTreeMap::new();

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

            if word_digit.contains_key(substring) {
                substring_vec.push(substring);
                indices.insert(i, word_digit.get(substring).unwrap().to_string());
            }
        }
    }
    let values = line.chars();
    let mut counter = 0;
    for c in values{
        if c.is_digit(10) {           
            indices.insert(counter, c.to_string());
        } 
        counter += 1;
    }   
    if let Some(first_entry) = indices.first_key_value() {
        let first_value = first_entry.1.to_string();

        if let Some(last_entry) = indices.last_key_value() {
            let last_value = last_entry.1.to_string();
            let concatenated = first_value + &last_value;
            return concatenated.parse::<u32>().unwrap();
        } else {
            println!("The BTreeMap is empty.");
        }
    } else {
        println!("The BTreeMap is empty.");
    }
    return 0
}
