/// * Time complexity: Best & Avg: O(n*logn), Worst: O(n*n).
/// * Space complexity: O(n*logn) in average case.
/// * Sorting in place: Yes.
/// * Stable: No.
pub fn quick_sort<T: Ord>(list: &mut [T]) {
    quick_sort_bound(list, 0, list.len())
}

fn quick_sort_bound<T: Ord>(list: &mut [T], start: usize, end: usize) {
    if end - start < 2 {
        return;
    }
    let partition_index = partition(list, start, end);
    quick_sort_bound(list, start, partition_index);
    quick_sort_bound(list, partition_index + 1, end);
}

fn partition<T: Ord>(list: &mut [T], start: usize, end: usize) -> usize {
    let mut p_index = start;
    let last_index = end - 1;

    for i in start..last_index {
        if list[i] < list[last_index] {
            list.swap(i, p_index);
            p_index += 1;
        }
    }

    list.swap(p_index, last_index);

    p_index
}

#[cfg(test)]
mod tests {
    use super::super::test_helpers;
    use super::quick_sort as sort;

    #[test]
    fn run_tests() {
        let tests = test_helpers::get_sorting_tests();

        for (input, result, msg) in &tests {
            let mut input = input.clone();
            sort(&mut input);
            assert_eq!(&input, result, "{}", msg);
        }
    }
}
