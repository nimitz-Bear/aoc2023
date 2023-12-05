use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    cmp::min
};

use regex::Regex;

fn main() {
    let lines = get_input("day3.txt");
    part1(lines);
}

fn part1(lines: Vec<String>) {
    let mut grid = vec![vec!['.'; lines[0].len()]; lines.len()];

    let mut sums: Vec<u32> = Vec::new();
 
    for (i, line) in lines.iter().enumerate() {
        grid[i] = line.chars().collect();
    }

    for (i, line) in lines.into_iter().enumerate() {
        let pattern = Regex::new(r"\d+").unwrap();

        for mat in pattern.find_iter(&line) {
            let (start, end) = (mat.start(), mat.end());
            
           
            println!("Match found at indices: ({}, {})", start, end);
            if is_number_valid(&grid, i, start, end - 1) {
                println!("Valid: {:?}", mat.as_str());
                sums.push(mat.as_str().parse().unwrap())

            } else {
                println!("Invalid: {:?}", mat.as_str());
            }
        }
    }

    println!("{}", sums.iter().sum::<u32>());

}

fn is_number_valid(grid: &[Vec<char>], i: usize, start: usize, end: usize) -> bool {
    for j in start..(end+1) {
        let neighbours: Vec<_> = get_neighbours(&grid, i, j);
        // println!("{:?}", neighbours);
        
        for x in neighbours {
            // if there is a single symbol, it's valid (i.e. not a digit or a .)
            if !(x.is_digit(10) || x =='.') {
                // println!("{}", x);
                return true;
            }
        }
    }
    false
}

fn get_neighbours(grid: &[Vec<char>], i: usize, j: usize) -> Vec<char> {
    let row_limit = grid.len();
    let column_limit = grid[0].len();

    let mut neighbours: Vec<char> = Vec::new();

    let minx = if ((i as isize -1 as isize) as isize) > 0 {i-1} else {0}; // the isizes are to deal with overflow errors, quite cursed
    for x in minx..min(i+2, row_limit) {

        let miny = if (j as isize -1 as isize) > 0 {j-1} else {0}; // the isizes are to deal with overflow errors, quite cursed
        for y in miny..min(j+2, column_limit) {
        
        
          if x != i || y != j {
            neighbours.push(grid[x][y]);
          }
        }
     }

    neighbours
}

fn part2() {}

fn get_input(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

