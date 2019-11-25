use super::binary_search::binary_search_bound;
use std::cmp;

/// Starts by checking the first value, then cuts off bigger and bigger chunks (exponentially
/// growing in size) of the list. Once it finds that the cut-off value is bigger than the one
/// searched for, it performs a binary search on the chunk.
///
/// Exponential searching is useful in two circumstances:
///
/// 1. The list is unbounded / infinite
/// 2. The searched for element is near the beginning of the list
///
/// [Wikipedia](https://en.wikipedia.org/wiki/Exponential_search)
pub fn exponential_search<T: Ord + Eq>(sorted_list: &[T], x: &T) -> Option<usize> {
    let size = sorted_list.len();

    if size == 0 {
        return None;
    }

    if sorted_list[0] == *x {
        return Some(0);
    }

    let mut bound = 1;

    while bound < size && sorted_list[bound] <= *x {
        bound *= 2;
    }

    let lo = bound / 2;
    let hi = cmp::min(bound, size);

    binary_search_bound(&sorted_list, x, lo, hi)
}

#[cfg(test)]
mod tests {
    use super::super::test_helpers;
    use super::exponential_search as search;

    #[test]
    fn run_tests() {
        let tests = test_helpers::get_search_tests();

        for (list, x, result, msg) in &tests {
            let out = search(&list, &x);
            assert_eq!(out, *result, "Searching for {} in {:?}. {}", &x, &list, msg);
        }
    }
}
