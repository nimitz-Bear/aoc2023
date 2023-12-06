use std::fs;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("full.txt").expect("Should have been able to read a file");
    let lines: Vec<&str> = input.split('\n').collect();
    println!("{:?}", lines);
    part_1(lines.as_slice());

    // let re = Regex::new(r"\d+").unwrap();
    // let times: Vec<_> = re.find_iter(lines[0]).map(|m| m.as_str().parse::<usize>().unwrap()).collect();
    // let records: Vec<_>  = re.find_iter(lines[1]).map(|m| m.as_str().parse::<usize>().unwrap()).collect();
}

fn part_1(lines: &[&str]) {
    let mut results: Vec<i32> = Vec::new();

    for line in lines {
        let mut win_count = 0;
        // remove the part before teh semi-colon (to ignore card number)
        let list: Vec<_> = line.split(':').collect();
        
        // split off into winning and my cards
        let segments: Vec<&str> = list[1].split('|').collect();
        
        let re = Regex::new(r"\d+").unwrap();
        let winning_cards: Vec<_> = re.find_iter(segments[0]).map(|m| m.as_str().parse::<usize>().unwrap()).collect();
        
        let my_cards: Vec<_> = re.find_iter(segments[1]).map(|m| m.as_str().parse::<usize>().unwrap()).collect();

        println!("{:?}", winning_cards);       
        println!("{:?}", my_cards);      

        for card in winning_cards {
            if my_cards.contains(&card) {
                win_count += 1;
                println!("Winning card: {}", card);
            }
        }
        if win_count > 0 {
            let base: i32 = 2;
            println!("Points: {:?}", base.pow(win_count-1));
            results.push(base.pow(win_count-1));
            
        }
  
    }
    
    println!("Part 1: {}",results.iter().fold(0, |accm, x| accm + x));
   
}

fn part_2() {

}