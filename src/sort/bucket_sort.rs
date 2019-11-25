/// Distributes elements into 10 buckets which it then sorts individually with insertion_sort
/// before gathering them back into the original array.
/// This implementation only sorts elements between 0 and 100.
pub fn bucket_sort(list: &mut [u64]) {
    use super::insertion_sort::insertion_sort;
    // This implementation looks clunky, but it's easy to follow and it completely sidesteps the
    // problem of moving values in and out of lists.

    // Step 1: Create 10 buckets
    let mut b0 = vec![];
    let mut b1 = vec![];
    let mut b2 = vec![];
    let mut b3 = vec![];
    let mut b4 = vec![];
    let mut b5 = vec![];
    let mut b6 = vec![];
    let mut b7 = vec![];
    let mut b8 = vec![];
    let mut b9 = vec![];

    // Step 2: Scatter elements into the buckets
    for i in 0..list.len() {
        let x = list[i];
        match x / 10 {
            0 => b0.push(x),
            1 => b1.push(x),
            2 => b2.push(x),
            3 => b3.push(x),
            4 => b4.push(x),
            5 => b5.push(x),
            6 => b6.push(x),
            7 => b7.push(x),
            8 => b8.push(x),
            9 => b9.push(x),
            _ => panic!("This function only sorts u64 between 0 and 100"),
        }
    }

    // Step 3: Sort each bucket individually.
    insertion_sort(&mut b0);
    insertion_sort(&mut b1);
    insertion_sort(&mut b2);
    insertion_sort(&mut b3);
    insertion_sort(&mut b4);
    insertion_sort(&mut b5);
    insertion_sort(&mut b6);
    insertion_sort(&mut b7);
    insertion_sort(&mut b8);
    insertion_sort(&mut b9);

    // Step 4: Gather. Concatinate the buckets and move the values back into the original list.
    [b0, b1, b2, b3, b4, b5, b6, b7, b8, b9]
        .iter()
        .flatten()
        .enumerate()
        .for_each(|(i, x)| {
            list[i] = *x;
        });
}

#[cfg(test)]
mod tests {
    use super::super::test_helpers;
    use super::bucket_sort as sort;

    #[test]
    fn run_tests() {
        let tests = test_helpers::get_sorting_tests();

        // We only care for the tests that run between 0 and 100.
        for (input, result, msg) in &tests[7..10] {
            // And we want them to be u64
            let mut u64_input: Vec<u64> = input.iter().map(|n| *n as u64).collect();
            let u64_result: Vec<u64> = result.iter().map(|n| *n as u64).collect();

            sort(&mut u64_input);

            assert_eq!(u64_input, u64_result, "{}", msg);
        }
    }
}
