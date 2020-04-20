use rand::seq::SliceRandom;
use rand::thread_rng;

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

    /// starting from first item, switch keys[i] keys[i+1] if keys[i] is bigger.
    /// This way, each iteration moves the biggest item to the right most position.
    fn bubble_sort(keys: &mut [Self]) {
        for i in 0..keys.len() {
            for j in 1..(keys.len() - i) {
                if keys[j] < keys[j-1] {
                    Sorting::swap(keys, j, j-1)
                }
            }
        }
    }

    /// <https://en.wikipedia.org/wiki/Shellsort>
    fn shellsort(keys: &mut [Self]) {
        let mut h = 1;
        while h < keys.len() / 3 {
            h = 3 * h + 1
        }

        while h >= 1 {
            for i in h..keys.len() {
                let key = keys[i].clone();
                let mut j = i;
                while j >= h && keys[j - h] > key {
                    keys[j] = keys[j - h].clone();
                    j -= h;
                }
                keys[j] = key;
            }

            h /= 3;
        }
    }

    /// <https://en.wikipedia.org/wiki/Quicksort>
    fn quicksort(keys: &mut [Self]) {
        if keys.len() == 0 {
            return;
        }
        // random shuffle keys
        let mut rng = thread_rng();
        keys.shuffle(&mut rng);
        // recursive sort fn
        sort(keys, 0, keys.len() - 1);
        fn sort<T: PartialOrd + Clone>(keys: &mut [T], lo: usize, hi: usize) {
            if hi <= lo {
                return;
            }
            let j = partition(keys, lo, hi);
            sort(keys, lo, j - 1);
            sort(keys, j + 1, hi);
        }
        // index lo to hi, keys[lo] is the pivot
        fn partition<T: PartialOrd + Clone>(keys: &mut [T], lo: usize, hi: usize) -> usize {
            let mut i = lo;
            let mut j = hi + 1;
            let pivot = keys[lo].clone();
            loop {
                loop {
                    i += 1;
                    if i == hi || keys[i] >= pivot {
                        break;
                    }
                }
                loop {
                    j -= 1;
                    if j == lo || pivot >= keys[j] {
                        break;
                    }
                }
                if i >= j {
                    break;
                }
                Sorting::swap(keys, i, j);
            }
            Sorting::swap(keys, j, lo);
            j
        }
    }




    #[doc(hidden)]
    fn swap(keys: &mut [Self], i: usize, j: usize) {
        let temp = keys[j].clone();
        keys[j] = keys[i].clone();
        keys[i] = temp;
    }
}

impl<T: PartialOrd + Clone> Sorting for T {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // let keys_original = [5, 4, 3, 6, 7, 8, 9, 0, 1, 2, 5, 4, 3, 6, 8, 0, 7, 5];
        let keys_original:Vec<&i32> = [5, 4, 3, 6, 7, 8, 9, 0, 1, 2, 5, 4, 3, 6, 8, 0, 7, 5].iter().collect();
        let mut sorted = keys_original.clone();
        sorted.sort_unstable();
        let sortings: Vec<fn(&mut [_])> = vec![
            Sorting::insertion_sort,
            Sorting::selection_sort,
            Sorting::shellsort,
            Sorting::quicksort,
            Sorting::bubble_sort
        ];
        for sort in sortings {
            let mut keys = keys_original.clone();
            sort(&mut keys);
            assert_eq!(sorted, keys);
        }
    }
}
