/// A variation of the binary search, where it looks at how big the value is and then tries to
/// search in the relevant area of the list. Say, the first item in the list is 0 and the last is
/// 100. If searching for 99, then it'll start by searching towards the end first.
///
/// The algorithm makes some assumptions about the values in the list. Namely that it can do
/// arithmetic on them. This assumption may not be true, and is a bit difficult to encode into
/// Rust's generic type system, so this implementation just assumes values of i64.
pub fn interpolation_search(sorted_list: &[i64], x: i64) -> Option<usize> {
    use std::cmp::Ordering;

    let mut lo = 0;
    let mut hi = sorted_list.len() - 1;

    while lo <= hi && x >= sorted_list[lo] && x <= sorted_list[hi] {
        if lo == hi {
            return if sorted_list[lo] == x { Some(lo) } else { None };
        }

        let pos = (lo as i64
            + ((x - sorted_list[lo]) * (hi - lo) as i64 / (sorted_list[hi] - sorted_list[lo])))
            as usize;

        match sorted_list[pos].cmp(&x) {
            Ordering::Equal => {
                return Some(pos);
            }
            Ordering::Less => {
                lo = pos + 1;
            }
            Ordering::Greater => {
                hi = pos - 1;
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::super::test_helpers;
    use super::interpolation_search as search;

    #[test]
    fn run_tests() {
        let tests = test_helpers::get_search_tests();

        for (list, x, result, msg) in &tests {
            let out = search(&list, *x);
            assert_eq!(out, *result, "Searching for {} in {:?}. {}", &x, &list, msg);
        }
    }
}
