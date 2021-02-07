use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Karatsuba Multiplication
fn karatsuba() {}

/// ```text
/// [2, 3] + [1,7] = [4,0]
/// ```
pub fn vec_decimal_add(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
    let len = std::cmp::max(a.len(), b.len());
    let mut vec = Vec::with_capacity(len + 1);
    let mut carry = 0u8;
    for i in 0..len {
        let v_a = a.len().checked_sub(1 + i).map(|index| a[index]).unwrap_or(0);
        let v_b = b.len().checked_sub(1 + i).map(|index| b[index]).unwrap_or(0);
        let sum = carry + v_a + v_b;
        if sum >= 10 {
            carry = 1;
            vec.push(sum % 10);
        } else {
            vec.push(sum);
            carry = 0;
        }
    }

    if (carry == 1) {
        vec.push(1);
    }

    vec.reverse();
    vec
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./examples/p1/input.txt")?;
    let numbers = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<u8>>())
        .collect::<Vec<_>>();

    println!("{:?}", numbers);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::vec_decimal_add;

    #[test]
    fn vec_decimal_add_works() {
        assert_eq!(vec_decimal_add(vec![], vec![]), vec![]);
        assert_eq!(vec_decimal_add(vec![1, 2], vec![3, 4]), vec![4, 6]);
        assert_eq!(vec_decimal_add(vec![1, 2], vec![3, 8]), vec![5, 0]);
        assert_eq!(vec_decimal_add(vec![1, 2], vec![9, 8]), vec![1, 1, 0]);
    }
}