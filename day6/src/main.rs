use std::fs;
use regex::Regex;

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let input = fs::read_to_string("full.txt").expect("Should have been able to read a file");
    let lines: Vec<_> = input.split('\n').collect();

    let re = Regex::new(r"\d+").unwrap();
    let times: Vec<_> = re.find_iter(lines[0]).map(|m| m.as_str().parse::<usize>().unwrap()).collect();
    let records: Vec<_>  = re.find_iter(lines[1]).map(|m| m.as_str().parse::<usize>().unwrap()).collect();

    let mut counts: Vec<u32> = Vec::new();

    // go through all the races
    for (i,time) in times.iter().enumerate() {
        let mut count = 0;
     
        // go through all possible seconds from 1 to time - 1
        for x in 1..*time {
            let distance = x*(time - x);
            if distance > records[i] {
                count +=1;
            }
        }
        println!("{}", count);
        counts.push(count);
    }

    println!("Part 1: {}", counts.iter().fold(1, |product, x| product * x));
}

fn part_2() {
    let input = fs::read_to_string("full.txt").expect("Should have been able to read a file");
    let lines: Vec<_> = input.split('\n').collect();

    let re = Regex::new(r"\d+").unwrap();
    let times: Vec<_> = re.find_iter(lines[0]).map(|m| m.as_str()).collect();
    let records: Vec<_>  = re.find_iter(lines[1]).map(|m| m.as_str()).collect();

    let race_time = times.join("").parse::<usize>().unwrap();
    let race_record = records.join("").parse::<usize>().unwrap();
    
    println!("{}", race_time);
    println!("{}", race_record);

    let mut lowest_mil = 0;

    // find the lowest way to beat the record
    for x in 1..race_time {
        let distance = x*(race_time - x);
            if distance > race_record {
                lowest_mil = x;
                break;
            }
    }

    // find the highest way to beat the record
    let mut highest_mil = 0;
     // find the lowest way to beat the record
     for x in (1..race_time).rev() {
        let distance = x*(race_time - x);
            if distance > race_record {
                highest_mil = x;
                break;
            }
    }

    println!("{}", lowest_mil);
    println!("{}", highest_mil);

    println!("Part 2: {}", highest_mil - lowest_mil + 1);

    

}