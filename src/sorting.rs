/// Implementation of sorting algorithms
pub trait Sorting
where
    Self: PartialOrd + Clone,
{
    /// Starting from the first item, insert the current key into
    /// the already sorted portion of the keys.
    /// Move the items in the sorted portion one place right if they
    /// are greater than the current key. Else insert the current key
    /// to the vacancy.
    fn insertion_sort(keys: &mut [Self]) {
        let f = PartialOrd::gt;

        for j in 1..keys.len() {
            let key = keys[j].clone();
            let mut i = j;
            while i > 0 && f(&keys[i - 1], &key) {
                // move value of keys[i-1] one place right
                keys[i] = keys[i - 1].clone();
                i -= 1;
            }
            // insertion happens here
            keys[i] = key;
        }
    }

    /// Select the smallest item in the unsorted portion of keys 
    /// and exchange it with the first item of the unsorted portion.
    /// Now this first item belongs to the sorted portion. 
    /// Repeat until no unsorted portion left.
    fn selection_sort(keys: &mut [Self]) {
        for i in 0..keys.len() {
            let mut min = i;
            for j in (i + 1)..keys.len() {
                if keys[j] < keys[min] {
                    min = j;
                }
            }
            let temp = keys[i].clone();
            keys[i] = keys[min].clone();
            keys[min] = temp;
        }
    }
}

impl<T: PartialOrd + Clone> Sorting for T {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let keys_original = [5, 4, 3, 6, 7, 8, 9, 0, 1, 2, 5, 4, 3, 6, 8, 0, 7, 5];
        let mut sorted = keys_original.clone();
        sorted.sort_unstable();
        let sortings: Vec<fn(&mut [_])> = vec![Sorting::insertion_sort, Sorting::selection_sort];
        for sort in sortings {
            let mut keys = keys_original.clone();
            sort(&mut keys);
            assert_eq!(sorted, keys);
        }
    }
}
