use std::cmp::Ordering;

/// Works by checked the value in the middle of the list to see if that's the one we are looking
/// for. If it is, great, if it's less then it repeats the procses with half above the middle, and
/// if it's greater, then it repeats with the lower half.
pub fn binary_search<T: Ord + Eq>(sorted_list: &[T], x: &T) -> Option<usize> {
    binary_search_bound(sorted_list, x, 0, sorted_list.len())
}

pub fn binary_search_bound<T: Ord + Eq>(
    list: &[T],
    x: &T,
    min: usize,
    max: usize,
) -> Option<usize> {
    // Once the list is empty, we return None
    if min >= max {
        return None;
    }

    let mid: usize = (min + max) / 2;

    match list[mid].cmp(&x) {
        Ordering::Equal => Some(mid),
        Ordering::Less => binary_search_bound(list, x, mid + 1, max),
        Ordering::Greater => binary_search_bound(list, x, min, mid),
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_helpers;
    use super::binary_search as search;

    #[test]
    fn run_tests() {
        let tests = test_helpers::get_search_tests();

        for (list, x, result, msg) in &tests {
            let out = search(&list, &x);
            assert_eq!(out, *result, "Searching for {} in {:?}. {}", &x, &list, msg);
        }
    }
}
