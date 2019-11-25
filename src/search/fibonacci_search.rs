use std::cmp;
use std::cmp::Ordering;

/// Uses the lower of a fibonacci triple (eg. 3, 5, and 8) as the middle point in a binary search.
/// In practice fibonacci Search search is rarely faster than a binary search.
///
/// Reasons that I have seen for using fibonacci search:
///
/// 1. Fibonacci Search divides given array in unequal parts
/// 2. Binary Search uses division operator to divide range. Fibonacci Search doesn’t use /, but
///    uses + and -. The division operator may be costly on some CPUs.
/// 3. Fibonacci Search examines relatively closer elements in subsequent steps. So when input
///    array is so big that it cannot fit in CPU cache or even in RAM, Fibonacci Search can be
///    useful.
///
/// Also [this answer](https://stackoverflow.com/questions/22877763#answer-22877947) on Stack
/// Overflow.
pub fn fibonacci_search<T: Ord + Eq>(sorted_list: &[T], x: &T) -> Option<usize> {
    let size = sorted_list.len();
    let mut fib2 = 0;
    let mut fib1 = 1;
    let mut fib = fib2 + fib1;

    while fib < size {
        fib2 = fib1;
        fib1 = fib;
        fib = fib2 + fib1;
    }

    let mut offset = 0;

    while fib > 1 {
        let index = cmp::min(offset + fib2, size) - 1;

        match sorted_list[index].cmp(&x) {
            Ordering::Less => {
                fib = fib1;
                fib1 = fib2;
                fib2 = fib - fib1;
                offset = index + 1;
            }
            Ordering::Greater => {
                fib = fib2;
                fib1 = fib1 - fib2;
                fib2 = fib - fib1;
            }
            Ordering::Equal => return Some(index),
        }
    }

    if fib1 == 1 && sorted_list[offset] == *x {
        return Some(offset);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::super::test_helpers;
    use super::fibonacci_search as search;

    #[test]
    fn run_tests() {
        let tests = test_helpers::get_search_tests();

        for (list, x, result, msg) in &tests {
            let out = search(&list, &x);
            assert_eq!(out, *result, "Searching for {} in {:?}. {}", &x, &list, msg);
        }
    }
}
