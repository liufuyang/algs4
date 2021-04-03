use std::cmp::Ordering;

/// Sort the input slice and return a copy of the slice elements as Vec.
/// Also return the number of inversions in the file given, where the i^{th}
/// row of the file indicates the i^{th} entry of an array.
pub fn sort<T>(input: &[T]) -> (Vec<T>, usize)
    where T: Clone + Ord
{
    if input.len() <= 1 {
        return (input.to_vec(), 0);
    }

    let (left, right) = input.split_at(input.len() / 2);
    let (left_sorted, left_inv_count) = sort(left);
    let (right_sorted, right_inv_count) = sort(right);

    let len = left_sorted.len() + right_sorted.len();
    let mut combined_vec = Vec::with_capacity(len);

    let mut l = 0usize;
    let mut r = 0usize;
    let mut inv_count_between_lr = 0usize; // inversion between the two part

    while l + r < len {
        match (left_sorted.get(l), right_sorted.get(r)) {
            (Some(left), Some(right)) => {
                match left.cmp(right) {
                    Ordering::Greater => {
                        combined_vec.push((*right).clone());
                        r = r + 1;
                        inv_count_between_lr = inv_count_between_lr + left_sorted.len() - l;
                    }
                    _ => {
                        combined_vec.push((*left).clone());
                        l = l + 1;
                    }
                }
            }
            (None, Some(right)) => {
                combined_vec.push((*right).clone());
                r = r + 1;
            }
            (Some(left), None) => {
                combined_vec.push((*left).clone());
                l = l + 1;
            }
            _ => break
        }
    }

    (combined_vec, left_inv_count + right_inv_count + inv_count_between_lr)
}

#[cfg(test)]
mod tests {
    use crate::merge_sort::sort;

    #[test]
    fn sort_works() {
        assert_eq!(sort(&[3, 5, 2, 6, 4, 1]), (vec![1, 2, 3, 4, 5, 6], 9));
        assert_eq!(sort(&[3, 5, 2, 4, 6, 1]), (vec![1, 2, 3, 4, 5, 6], 8));
        assert_eq!(sort(&[4, 5, 6, 1, 2, 3]), (vec![1, 2, 3, 4, 5, 6], 9));
        assert_eq!(sort(&[5, 4, 3, 2, 1]), (vec![1, 2, 3, 4, 5], 4 + 3 + 2 + 1));
        assert_eq!(sort(&[5, ]), (vec![5], 0));
        assert_eq!(sort(&[1, 2, 3, 4, 5, 6]), (vec![1, 2, 3, 4, 5, 6], 0));
    }
}