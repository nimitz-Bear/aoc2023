use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

use regex::Regex;

fn main() {
    let lines = get_input("day2.txt");

    for (i, line) in lines.iter().enumerate() {
        let start = line.find(':');
        println!("{}", line);

        match start {
            Some(x) => {
                // find the part after the ':'
                let ss: String = line.chars().skip(x + 2).collect();

                // check each set
                let game = ss.split(";");


                for set in game {
                    println!("{}", is_set_valid(set.to_string()));
                }

            }
            None    => println!("Somehow, : not found"),
        }
    }
} 



fn is_set_valid(line: String) -> bool {
    let colors = line.split(",");

    // each color is seperated by 
    for color in colors {
        // use regex to find the last number in regex format


        let re = Regex::new(r"\d+");
        
    

        // Check if the regex compilation was successful
    if let Ok(re) = re {
            // Print the matched portion
            let count = re.find_iter(color).last();

            match count {
                Some(a) => {
                    let test: u32 = a.as_str().parse().unwrap();

                    if color.contains("blue") && test > 14 {

                        return false;
                    } else if color.contains("red") && test > 13 {
               
                        return false;
                    } else if color.contains("green") && test > 12 {
                      
                        return false;
                    } 
                    
                },
                None => println!("Count")
            }

    }


    }


    true

    
}

fn get_input(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}