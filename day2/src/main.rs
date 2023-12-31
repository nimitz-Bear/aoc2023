use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

use regex::Regex;

fn main() {
    let lines = get_input("day2.txt");
    // part_1(lines);
    part_2(lines);
}

fn part_2(lines: Vec<String>) {
    let mut powers: Vec<u32> = Vec::new();

    for line in lines {
        println!("{}", line);
        let mut blues: Vec<u32> = Vec::new();
        let mut reds: Vec<u32> = Vec::new();
        let mut greens: Vec<u32> = Vec::new();

        let sets = line.split(";");

        for set in sets {
            println!("{}", set);

            let sections = set.split(",");

            for section in sections {
                let re = Regex::new(r"\d+");
                // Check if the regex compilation was successful
                if let Ok(re) = re {
                    // print the matched portion
                    let count = re.find_iter(section).last();

                    match count {
                        Some(a) => {
                            let test: u32 = a.as_str().parse().unwrap();

                            if section.contains("blue") {
                                blues.push(test);
                            } else if section.contains("red") {
                                reds.push(test);
                            } else if section.contains("green") {
                                greens.push(test);
                            }
                        }
                        None => println!("Error: No count found"),
                    }
                }
            }
        }

        println!("blues {}", blues.len());
        println!("reds {}", reds.len());
        println!("greens {}", greens.len());

        let mut minBlue = blues.iter().max();
        match minBlue {
            Some(min) => println!("Min blue value: {}", min),
            None => minBlue = Some(&1),
        }

        let mut minRed = reds.iter().max();
        match minRed {
            Some(min) => println!("Min red value: {}", min),
            None => minRed = Some(&1),
        }

        let mut minGreen = greens.iter().max();
        match minGreen {
            Some(min) => println!("Min green value: {}", min),
            None => minGreen = Some(&1),
        }

        let test = minBlue.unwrap() * minRed.unwrap() * minGreen.unwrap();
        println!("{}", test);

        // by now, all of the values should be at least 1
        powers.push(test);
    }
    println!("{}", powers.iter().sum::<u32>());
}

fn part_1(lines: Vec<String>) {
    let mut ids: Vec<usize> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        let start = line.find(':');
        println!("{}", line);

        if is_game_valid(line, start) {
            println!("{} is valid", i + 1);
            ids.push(i + 1);
        }
    }

    println!("{}", ids.iter().sum::<usize>());
}

fn is_game_valid(line: &String, start: Option<usize>) -> bool {
    match start {
        Some(x) => {
            // find the part after the ':'
            let ss: String = line.chars().skip(x + 2).collect();

            // check each set
            let game = ss.split(";");

            for set in game {
                let result = is_set_valid(set.to_string());

                if !result {
                    return result;
                }
            }

            return true;
        }
        None => {
            println!("Somehow, : not found");
            false
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
                    } else if color.contains("red") && test > 12 {
                        return false;
                    } else if color.contains("green") && test > 13 {
                        return false;
                    }
                }
                None => println!("Count"),
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
