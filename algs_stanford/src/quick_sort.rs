use std::cmp::Ordering::*;

/// Simple quick sort input slice and return the number of comparisons.
/// Using first element as pivot (not good for already ordered arrays or full of duplicates)
/// Return cmp count
pub fn sort_1<T>(a: &mut [T]) -> usize
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
            Less => {
                a.swap(i, j);
                i = i + 1;
            }
            _ => ()
        }
    }
    let pivot = i - 1;
    a.swap(anchor, pivot);

    let c1 = sort_1(&mut a[0..pivot]);
    let c2 = sort_1(&mut a[pivot + 1..len]);

    c1 + c2 + a.len() - 1
}

/// Simple quick sort input slice and return the number of comparisons.
/// Using last element as pivot (not good for already ordered arrays or full of duplicates)
/// Return cmp count
pub fn sort_2<T>(a: &mut [T]) -> usize
    where T: Ord
{
    let len = a.len();
    if a.len() <= 1 {
        return 0;
    }

    let anchor = 0usize;
    a.swap(anchor, len - 1); // Using last element as pivot
    let mut i = 1usize;
    for j in 1..len {
        match &a[j].cmp(&a[anchor]) {
            Less => {
                a.swap(i, j);
                i = i + 1;
            }
            _ => ()
        }
    }
    let pivot = i - 1;
    a.swap(anchor, pivot);

    let c1 = sort_2(&mut a[0..pivot]);
    let c2 = sort_2(&mut a[pivot + 1..len]);

    c1 + c2 + a.len() - 1
}

/// Simple quick sort input slice and return the number of comparisons.
/// Using a simple median of `[first, mid, last]` as pivot value.
/// Return cmp count
pub fn sort_3<T>(a: &mut [T]) -> usize
    where T: Ord
{
    let len = a.len();
    if a.len() <= 1 {
        return 0;
    }

    let anchor = 0usize;

    // select median
    let first = 0usize;
    let last = len - 1;
    let mid = last / 2;
    let median = match (&a[first].cmp(&a[mid]), &a[mid].cmp(&a[last]), &a[first].cmp(&a[last])) {
        (Less, Less, _) => mid,
        (Less, Greater, Less) => last,
        (Less, Greater, Greater) => first,
        (Greater, Greater, _) => mid,
        (Greater, Less, Less) => first,
        (Greater, Less, Greater) => last,
        (Equal, Equal, Equal) => mid,
        (Equal, _, _) => first,
        (_, Equal, _) => mid,
        (_, _, Equal) => last
    };

    a.swap(anchor, median);

    let mut i = 1usize;
    for j in 1..len {
        match &a[j].cmp(&a[anchor]) {
            Less => {
                a.swap(i, j);
                i = i + 1;
            }
            _ => ()
        }
    }
    let pivot = i - 1;
    a.swap(anchor, pivot);

    let c1 = sort_3(&mut a[0..pivot]);
    let c2 = sort_3(&mut a[pivot + 1..len]);

    c1 + c2 + a.len() - 1
}

#[cfg(test)]
mod tests {
    use crate::quick_sort::*;

    #[test]
    fn sort_1_works() {
        let mut a = vec![3, 5, 2, 6, 4, 1];
        let c = sort_1(&mut a);
        assert_eq!(vec![1, 2, 3, 4, 5, 6], a);
        assert_eq!(9, c); // 5 + 1 + 2 + 1

        let mut a = vec![1, 2, 3, 4, 5, 6];
        let c = sort_1(&mut a);
        assert_eq!(vec![1, 2, 3, 4, 5, 6], a);
        assert_eq!(15, c); // 5 + 4 + 3 + 2 + 1
    }

    #[test]
    fn sort_2_works() {
        let mut a = vec![3, 5, 2, 6, 4, 1];
        let c = sort_2(&mut a);
        assert_eq!(vec![1, 2, 3, 4, 5, 6], a);
        assert_eq!(11, c);

        let mut a = vec![1, 2, 3, 4, 5, 6];
        let c = sort_2(&mut a);
        assert_eq!(vec![1, 2, 3, 4, 5, 6], a);
        assert_eq!(15, c); // 5 + 4 + 3 + 2 + 1
    }

    #[test]
    fn sort_3_works() {
        let mut a = vec![3, 5, 2, 6, 4, 1];
        let c = sort_3(&mut a);
        assert_eq!(vec![1, 2, 3, 4, 5, 6], a);
        assert_eq!(9, c); // 5 + 3 + 1

        let mut a = vec![1, 2, 3, 4, 5, 6];
        let c = sort_3(&mut a);
        assert_eq!(vec![1, 2, 3, 4, 5, 6], a);
        assert_eq!(9, c); // 5 + 1 + 2 + 1
    }
}