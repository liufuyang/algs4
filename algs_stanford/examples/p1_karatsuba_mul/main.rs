use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::char::from_digit;

/// Karatsuba Multiplication
/// x = 10^(n/2)a + b
/// y = 10^(n/2)c + d,
/// x * y = 10^n ac + 10^(n/2)(ad + bc) + bd
/// ac ---------------------------------------(t1)
/// bd ---------------------------------------(t2)
/// ad + bc = (a + b)(c + d) - ac - bd -------(t3)
/// x * y = 10^n t1 + 10^(n/2)(t3) + t2
pub fn karatsuba_mul(v1: &[u8], v2: &[u8]) -> Vec<u8> {
    let mut v1 = v1;
    let mut v2 = v2;
    let hold;
    if v1.len() > v2.len() {
        hold = [&vec![0; v1.len() - v2.len()], v2].concat();
        v2 = &hold;
    } else if v1.len() < v2.len() {
        hold = [&vec![0; v2.len() - v1.len()], v1].concat();
        v1 = &hold;
    }

    assert_eq!(v1.len(), v2.len());
    // println!("v1:{:?}, v2:{:?}", v1, v2);

    if v1.len() >= 2 {
        let n = v1.len();
        let n_half = n / 2;
        let (a, b) = v1.split_at(n_half);
        let (c, d) = v2.split_at(n_half);
        let ac = karatsuba_mul(a, c);
        let bd = karatsuba_mul(b, d);
        let a_and_b_mul_c_and_d = karatsuba_mul(&vec_decimal_add(a, b), &vec_decimal_add(c, d));
        let t3 = vec_decimal_sub(&vec_decimal_sub(&a_and_b_mul_c_and_d, &ac), &bd);

        // handle the case of odd n, where n/2 = 1, for example, n=3 will make n_half=1, n_second_half=2
        // use 2 * n_second_half to scale ac and t3
        let n_second_half = n - n_half;
        let mut first = ac;
        for _ in 0..2 * n_second_half {
            first.push(0);
        }

        let mut second = t3;
        for _ in 0..n_second_half {
            second.push(0);
        }

        return vec_decimal_add(&vec_decimal_add(&first, &second), &bd);
    }
    if v1.is_empty() || v2.is_empty() {
        return Vec::new();
    }
    let m = v1[0] * v2[0];
    if m >= 10 {
        let carry = m / 10;
        let remain = m % 10;
        return vec![carry, remain];
    } else {
        return vec![m];
    }
}

/// ```text
/// [2, 3] + [1,7] = [4,0]
/// ```
pub fn vec_decimal_add(a: &[u8], b: &[u8]) -> Vec<u8> {
    let len = std::cmp::max(a.len(), b.len());
    let mut vec = Vec::with_capacity(len + 1);
    let mut carry = 0u8;
    for i in 0..len {
        let v_a = a
            .len()
            .checked_sub(1 + i)
            .map(|index| a[index])
            .unwrap_or(0);
        let v_b = b
            .len()
            .checked_sub(1 + i)
            .map(|index| b[index])
            .unwrap_or(0);
        let sum = carry + v_a + v_b;
        if sum >= 10 {
            carry = 1;
            vec.push(sum % 10);
        } else {
            vec.push(sum);
            carry = 0;
        }
    }

    if carry == 1 {
        vec.push(1);
    }

    while vec.len() > 1 && vec.ends_with(&[0]) {
        vec.pop();
    }
    vec.reverse();

    // println!("add a:{:?}, b:{:?} -> {:?}", a, b, vec);
    vec
}

/// ```text
/// [2, 3] - [1,7] = [0,6]
/// ```
pub fn vec_decimal_sub(a: &[u8], b: &[u8]) -> Vec<u8> {
    assert!(a.len() >= b.len() && a.len() > 0);

    let len = std::cmp::max(a.len(), b.len());
    let mut vec = Vec::with_capacity(len);
    let mut carry = 0u8;
    for i in 0..len {
        let v_a = a
            .len()
            .checked_sub(1 + i)
            .map(|index| a[index])
            .unwrap_or(0);
        let v_b = b
            .len()
            .checked_sub(1 + i)
            .map(|index| b[index])
            .unwrap_or(0);

        let to_sub = v_b + carry;
        if v_a < to_sub {
            carry = 1;
            vec.push(10 + v_a - to_sub);
        } else {
            vec.push(v_a - to_sub);
            carry = 0;
        }
    }

    assert_eq!(carry, 0, "sub: a is smaller than b?");
    while vec.len() > 1 && vec.ends_with(&[0]) {
        vec.pop();
    }
    vec.reverse();

    // println!("sub a:{:?}, b:{:?} -> {:?}", a, b, vec);
    vec
}

#[cfg(test)]
mod tests {
    use super::karatsuba_mul;
    use super::{vec_decimal_add, vec_decimal_sub};

    #[test]
    fn vec_decimal_add_works() {
        assert_eq!(vec_decimal_add(&vec![], &vec![]), vec![]);
        assert_eq!(vec_decimal_add(&vec![1, 2], &vec![3, 4]), vec![4, 6]);
        assert_eq!(vec_decimal_add(&vec![1, 2], &vec![3, 8]), vec![5, 0]);
        assert_eq!(vec_decimal_add(&vec![1, 2], &vec![9, 8]), vec![1, 1, 0]);
        assert_eq!(vec_decimal_add(&vec![1, 2, 3], &vec![9, 8]), vec![2, 2, 1]);
    }

    #[test]
    fn vec_decimal_sub_works() {
        assert_eq!(vec_decimal_sub(&vec![3, 4], &vec![1, 2]), vec![2, 2]);
        assert_eq!(vec_decimal_sub(&vec![1, 0, 0], &vec![1, 2]), vec![8, 8]);
        assert_eq!(vec_decimal_sub(&vec![0], &vec![0]), vec![0]);
    }

    #[test]
    #[should_panic]
    fn vec_decimal_sub_should_panic() {
        assert_eq!(vec_decimal_sub(&vec![0], &vec![1]), vec![0]);
    }

    #[test]
    fn karatsuba_mul_works() {
        assert_eq!(karatsuba_mul(&vec![2], &vec![4]), vec![8]);
        assert_eq!(karatsuba_mul(&vec![2], &vec![7]), vec![1, 4]);
        assert_eq!(karatsuba_mul(&vec![1, 2], &vec![3, 4]), vec![4, 0, 8]);
        assert_eq!(karatsuba_mul(&vec![1, 2], &vec![3]), vec![3, 6]);
        assert_eq!(
            karatsuba_mul(&vec![1, 2, 3, ], &vec![5, 6, 7, ]),
            vec![6, 9, 7, 4, 1]
        );
        assert_eq!(
            karatsuba_mul(&vec![1, 2, 3, 4], &vec![5, 6, 7, 8]),
            vec![7, 0, 0, 6, 6, 5, 2]
        );
        assert_eq!(
            karatsuba_mul(&vec![1, 2, 3, ], &vec![5, 6, 7, 8, 9]),
            vec![6, 9, 8, 5, 0, 4, 7]
        );
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./examples/p1_karatsuba_mul/input.txt")?;
    let numbers = BufReader::new(file)
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<_>>();

    println!(
        "{}",
        karatsuba_mul(&numbers[0], &numbers[1])
            .into_iter()
            .map(|n| from_digit(n as u32, 10).unwrap_or('_'))
            .collect::<String>()
    );
    Ok(())
    // 8539734222673567065463550869546574495034888535765114961879601127067743044893204848617875072216249073013374895871952806582723184
}
