use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use algs_stanford::merge_sort::sort;


fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./examples/p2_count_inversions/integerArray.txt")?;
    let numbers = BufReader::new(file)
        .lines()
        .map(|line| {
            line.unwrap().parse::<usize>().unwrap()
        })
        .collect::<Vec<_>>();

    println!("{}", numbers.len());
    println!("{}", sort(numbers.as_slice()).1); // 2407905288 ???
    Ok(())
}