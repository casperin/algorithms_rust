type Test = (Vec<i64>, i64, Option<usize>, &'static str);

pub fn get_search_tests() -> [Test; 9] {
    [
        (vec![0, 2, 3, 4, 5, 9], 2, Some(1), "Base case"),
        (vec![4], 4, Some(0), "One element"),
        (vec![4, 6], 4, Some(0), "Two elements"),
        (vec![4, 6], 6, Some(1), "Two elements, latter"),
        (vec![4, 6, 10, 23, 53, 100], 5, None, "Not found"),
        (vec![0, 2, 3, 4, 5, 9], 2, Some(1), "Another base case"),
        (
            vec![4, 6, 10, 23, 53, 100, 123, 132, 142, 231, 232, 259, 309],
            231,
            Some(9),
            "More numbers",
        ),
        (
            vec![4, 6, 10, 23, 53, 100, 123, 132, 142, 231, 232, 259, 309],
            400,
            None,
            "Out of the list",
        ),
        (
            vec![4, 6, 10, 23, 53, 100, 123, 132, 142, 231, 232, 259, 309],
            200,
            None,
            "Within list, but not found",
        ),
    ]
}
