/// ## Example
/// ```
/// let mut v = vec![45, 23, 98, 0, 2];
/// algorithms::sort::merge_sort(&mut v);
/// assert_eq!(v, vec![0, 2, 23, 45, 98]);
/// ```
pub fn merge_sort<T: std::marker::Copy + Ord>(list: &mut [T]) {
    merge_sort_section(list, 0, list.len())
}

fn merge_sort_section<T: std::marker::Copy + Ord>(list: &mut [T], start: usize, end: usize) {
    if start + 1 >= end {
        return; // if less than two items, noop
    }

    let mid = (start + end) / 2;

    // Split vector into left and right and sort those recursively
    merge_sort_section(list, start, mid); // sort left side
    merge_sort_section(list, mid, end); // sort right side

    let mut left_index = start;
    let mut right_index = mid;
    let mut buffer = Vec::with_capacity(end - start);

    // While we have items in both left and right vector we push the lowest of them onto the buffer
    while left_index < mid && right_index < end {
        let left_item = list[left_index];
        let right_item = list[right_index];

        if left_item < right_item {
            buffer.push(left_item);
            left_index += 1;
        } else {
            buffer.push(right_item);
            right_index += 1;
        }
    }

    // Push whatever is left of the left (or the right) vector
    while left_index < mid {
        buffer.push(list[left_index]);
        left_index += 1;
    }

    while right_index < end {
        buffer.push(list[right_index]);
        right_index += 1;
    }

    // Items in the buffer are sorted, so we update the section of the original array that we are
    // dealing with to reflect the buffer
    buffer.iter().enumerate().for_each(|(i, item)| {
        list[i + start] = *item;
    });
}

#[cfg(test)]
mod tests {
    use super::super::test_helpers;
    use super::merge_sort as sort;

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
