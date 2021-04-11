use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use algs_stanford::quick_sort::*;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./examples/p3_quick_sort/quicksort.txt")?;
    let numbers_raw = BufReader::new(file)
        .lines()
        .map(|line| {
            line.unwrap().parse::<usize>().unwrap()
        })
        .collect::<Vec<_>>();

    println!("{}", numbers_raw.len());

    let mut numbers = numbers_raw.clone();
    println!("sort_1 comparison count: {}", sort_1(numbers.as_mut_slice())); // q1: 162085 ???

    let mut numbers = numbers_raw.clone();
    println!("sort_2 comparison count: {}", sort_2(numbers.as_mut_slice())); // q1: 164123 ???

    let mut numbers = numbers_raw.clone();
    println!("sort_3 comparison count: {}", sort_3(numbers.as_mut_slice())); // q1: 138382 ???
    Ok(())
}