/// Classic simple sorting algorithm that is only ever worth it on very small lists.
pub fn selection_sort<T: Ord>(list: &mut [T]) {
    for i in 0..list.len() {
        let mut min_index = i;

        // Go through the rest of the list and find the index of actual lowest value
        for j in i + 1..list.len() {
            if list[j] < list[min_index] {
                min_index = j;
            }
        }

        // Then swap the current element and the one with the lowest index
        list.swap(i, min_index);
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_helpers;
    use super::selection_sort as sort;

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
