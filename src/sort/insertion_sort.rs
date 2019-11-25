/// For small or nearly sorted lists. This is often used in conjunction with other sorts, when
/// dealing with small lists.
///
/// Works by taking each element (starting with second element) and moving it towards the front of
/// the list until it reaches an element that is lower than itself.
///
/// Best case (an already sorted list) is O(n).
pub fn insertion_sort<T: Ord>(list: &mut [T]) {
    for i in 1..list.len() {
        for j in (0..i).rev() {
            if list[j] <= list[j + 1] {
                break;
            }
            list.swap(j, j + 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_helpers;
    use super::insertion_sort as sort;

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
