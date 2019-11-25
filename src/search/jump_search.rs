use std::cmp;

/// Jump Search is a glorified linear search. It is only useful if you know that the item you are
/// looking for is among the first ones in the list. If that isn't the case, then binary search
/// will have much better performance.
///
/// It checks the first number for the value. If not there, it jumps sqrt(list.length) forward and
/// checks again. When the checked value is lower than what we are searching for, we know we passed
/// it. So we do a linear search (from the previous value up to the current value) to find it.
pub fn jump_search<T: Ord + Eq>(sorted_list: &[T], x: &T) -> Option<usize> {
    let length = sorted_list.len();
    let mut step = (length as f64).sqrt() as usize;
    let mut prev = 0;

    // While the checked value is lower than the one we are searching for, we keep jumping.
    while &sorted_list[cmp::min(length, step) - 1] < x {
        prev = step;
        step += (length as f64).sqrt() as usize;

        // If we reached the end of the list and we are still in this loop, then x is higher than
        // any number in the list.
        if prev >= length {
            return None;
        }
    }

    // We passed our value. We know that the previous value is lower (that's why it's the previous
    // value), so we just increment `prev` as long as that is true.
    while &sorted_list[prev] < x {
        prev += 1;

        // If we reach the end of our little chunk and the checked value is still lower than x,
        // then we did not find x.
        if prev == cmp::min(length, step) {
            return None;
        }
    }

    if &sorted_list[prev] == x {
        Some(prev)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_helpers;
    use super::jump_search as search;

    #[test]
    fn run_tests() {
        let tests = test_helpers::get_search_tests();

        for (list, x, result, msg) in &tests {
            let out = search(&list, &x);
            assert_eq!(out, *result, "Searching for {} in {:?}. {}", &x, &list, msg);
        }
    }
}
