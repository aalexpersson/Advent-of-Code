use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        //println!("{:?}", line.unwrap());
        let value = line.unwrap();
        count(value);
    }

    Ok(())
    
}

fn count(line: String){
    let max_red = 12;
    let max_geen = 13;
    let max_blue = 14;

    let game = line.split(":").next().unwrap();
    let game_id = game.rsplit_once(" ").unwrap().1;

    let game_draws = line.strip_prefix(&(game.to_owned() + ": ")).unwrap();
    let draws = game_draws.split(";").collect::<Vec<&str>>();
    for c in draws{
        println!("{}", c) 
    }

    //println!("{:?}", draws);
}