use std::cmp::Ordering;

/// quick sort input slice and return the number of comparisons
/// return cmp count
pub fn sort<T>(a: &mut [T]) -> usize
    where T: Ord
{
    let len = a.len();
    if a.len() <= 1 {
        return 0;
    }

    let anchor = 0usize;
    let mut i = 1usize;
    for j in 1..len {
        match &a[j].cmp(&a[anchor]) {
            Ordering::Less => {
                a.swap(i, j);
                i = i + 1;
            }
            _ => ()
        }
    }
    let pivot = i - 1;
    a.swap(anchor, pivot);

    let c1 = sort(&mut a[0..pivot]);
    let c2 = sort(&mut a[pivot + 1..len]);

    c1 + c2 + a.len()
}

#[cfg(test)]
mod tests {
    use crate::quick_sort::sort;

    #[test]
    fn sort_works() {
        let mut a = vec![3, 5, 2, 6, 4, 1];
        let c =  sort(&mut a);
        assert_eq!(vec![1, 2, 3, 4, 5, 6], a);
        assert_eq!(13, c); // 6 + 2 + 3 + 2
        // assert_eq!(sort(&[3, 5, 2, 4, 6, 1]), (vec![1, 2, 3, 4, 5, 6], 8));
        // assert_eq!(sort(&[4, 5, 6, 1, 2, 3]), (vec![1, 2, 3, 4, 5, 6], 9));
        // assert_eq!(sort(&[5, 4, 3, 2, 1]), (vec![1, 2, 3, 4, 5], 4 + 3 + 2 + 1));
        // assert_eq!(sort(&[5, ]), (vec![5], 0));
        // assert_eq!(sort(&[1, 2, 3, 4, 5, 6]), (vec![1, 2, 3, 4, 5, 6], 0));
    }
}