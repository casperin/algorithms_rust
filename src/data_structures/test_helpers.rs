type Test = (Vec<i64>, Vec<i64>, &'static str);

pub fn get_random_numbers() -> [Test; 10] {
    [
        (vec![], vec![], "Empty list"),
        (vec![3], vec![3], "One element"),
        (vec![5, 3], vec![3, 5], "Two elements"),
        (vec![5, 5], vec![5, 5], "Two equal elements"),
        (vec![1, 2, 3], vec![1, 2, 3], "Three sorted elements"),
        (
            vec![3, 2, 5, 9, 0, 4],
            vec![0, 2, 3, 4, 5, 9],
            "Normal case",
        ),
        (
            vec![100, 90, 50, 14, 9, 7, 3],
            vec![3, 7, 9, 14, 50, 90, 100],
            "Reversed",
        ),
        (
            vec![
                32, 35, 75, 17, 12, 93, 37, 67, 16, 22, 47, 93, 43, 90, 57, 23, 89, 32, 16, 98, 40,
                73, 42, 64, 46, 85, 4, 25, 30, 57,
            ],
            vec![
                4, 12, 16, 16, 17, 22, 23, 25, 30, 32, 32, 35, 37, 40, 42, 43, 46, 47, 57, 57, 64,
                67, 73, 75, 85, 89, 90, 93, 93, 98,
            ],
            "30 numbers between 0 and 100",
        ),
        (
            vec![
                60, 45, 68, 39, 94, 0, 54, 40, 42, 39, 48, 67, 37, 42, 59, 56, 59, 51, 3, 31, 47,
                29, 84, 58, 69, 87, 25, 83, 71, 28,
            ],
            vec![
                0, 3, 25, 28, 29, 31, 37, 39, 39, 40, 42, 42, 45, 47, 48, 51, 54, 56, 58, 59, 59,
                60, 67, 68, 69, 71, 83, 84, 87, 94,
            ],
            "30 numbers between 0 and 100, 2",
        ),
        (
            vec![
                15, 21, 35, 53, 9, 85, 44, 21, 82, 85, 48, 56, 70, 10, 39, 61, 68, 37, 0, 20, 91,
                1, 43, 38, 72, 91, 96, 51, 29, 68, 56, 28, 22, 65, 57, 23, 29, 87, 94, 23, 56, 80,
                95, 18, 75, 27, 82, 87, 54, 72, 23, 91, 37, 73, 12, 61, 1, 63, 77, 12, 65, 82, 71,
                73, 51, 10, 56, 33, 54, 26, 66, 47, 6, 43, 43, 14, 18, 37, 66, 89, 12, 86, 32, 29,
                66, 37, 50, 55, 27, 52, 4, 91, 57, 8, 34, 78, 53, 54, 10, 75, 3, 30, 21, 73, 70,
                29, 65, 73, 42, 39, 48, 5, 52, 76, 75, 27, 40, 31, 13, 81, 91, 38, 77, 93, 87, 2,
                25, 37, 44, 75, 23, 63, 88, 20, 61, 75, 91, 29, 60, 5, 99, 41, 70, 11, 18, 95, 66,
                59, 0, 84, 15, 12, 16, 83, 87, 29, 6, 76, 32, 36, 59, 90, 60, 22, 60, 95, 87, 37,
                9, 66, 4, 59, 98, 59, 93, 35, 2, 44, 67, 43, 27, 52, 14, 74, 13, 32, 33, 61, 7, 32,
                83, 54, 90, 72, 18, 94, 57, 21, 26, 20, 5, 53, 93, 44, 78, 61, 94, 35, 23, 50, 14,
                34, 95, 31, 43, 0, 4, 80, 60, 73, 47, 66, 61, 76, 47, 47, 77, 6, 10, 33, 65, 45,
                49, 80, 37, 33, 80, 21, 29, 72, 43, 89, 6, 26, 37, 36, 59, 56, 34, 33, 77, 38, 38,
                47, 45, 19, 92, 78, 18, 94, 38, 3, 81, 89, 0, 16, 0, 61, 96, 76, 27, 56, 4, 84, 45,
                85, 76, 49, 82, 57, 73, 7, 41, 19, 97, 29, 34, 27, 68, 7, 26, 82, 39, 35, 88, 10,
                49, 26, 52, 31, 85, 69, 1, 49, 37, 75, 16, 40, 77, 66, 10, 41, 39, 90, 14, 2, 61,
                44, 88, 47, 66, 76, 13, 22, 5, 53, 99, 62, 36, 4, 76, 62, 63, 78, 92, 33, 25, 38,
                44, 6, 61, 19, 83, 81, 79, 44, 13, 6, 69, 85, 80, 85, 21, 83, 53, 31, 88, 26, 0,
                67, 15, 1, 63, 46, 66, 60, 93, 88, 73, 60, 74, 10, 69, 24, 39, 56, 39, 78, 28, 90,
                42, 0, 97, 76, 56, 75, 96, 9, 75, 24, 69, 39, 92, 64, 0, 35, 98, 53, 81, 68, 5, 52,
                34, 98, 54, 75, 92, 8, 65, 0, 93, 55, 86, 16, 73, 58, 60, 20, 25, 69, 65, 23, 29,
                36, 27, 44, 65, 3, 8, 20, 31, 89, 80, 52, 26, 48, 10, 14, 53, 79, 8, 95, 85, 18,
                70, 82, 83, 51, 59, 77, 49, 44, 10, 36, 14, 45, 80, 68, 49, 9, 54, 51, 50, 27, 60,
                44, 23, 17, 68, 13, 65, 1, 10, 67, 13, 67, 69, 63, 47, 37, 61, 17, 97, 19, 81, 22,
                57, 40, 9, 9, 10, 65, 25, 18, 46, 91, 36, 2, 56, 32,
            ],
            vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 4, 4, 4, 4, 4, 5, 5,
                5, 5, 5, 6, 6, 6, 6, 6, 6, 7, 7, 7, 8, 8, 8, 8, 9, 9, 9, 9, 9, 9, 10, 10, 10, 10,
                10, 10, 10, 10, 10, 10, 10, 11, 12, 12, 12, 12, 13, 13, 13, 13, 13, 13, 14, 14, 14,
                14, 14, 14, 15, 15, 15, 16, 16, 16, 16, 17, 17, 18, 18, 18, 18, 18, 18, 18, 19, 19,
                19, 19, 20, 20, 20, 20, 20, 21, 21, 21, 21, 21, 21, 22, 22, 22, 22, 23, 23, 23, 23,
                23, 23, 23, 24, 24, 25, 25, 25, 25, 26, 26, 26, 26, 26, 26, 26, 27, 27, 27, 27, 27,
                27, 27, 27, 28, 28, 29, 29, 29, 29, 29, 29, 29, 29, 29, 30, 31, 31, 31, 31, 31, 32,
                32, 32, 32, 32, 33, 33, 33, 33, 33, 33, 34, 34, 34, 34, 34, 35, 35, 35, 35, 35, 36,
                36, 36, 36, 36, 36, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 38, 38, 38, 38, 38, 38,
                39, 39, 39, 39, 39, 39, 39, 40, 40, 40, 41, 41, 41, 42, 42, 43, 43, 43, 43, 43, 43,
                44, 44, 44, 44, 44, 44, 44, 44, 44, 44, 45, 45, 45, 45, 46, 46, 47, 47, 47, 47, 47,
                47, 47, 48, 48, 48, 49, 49, 49, 49, 49, 49, 50, 50, 50, 51, 51, 51, 51, 52, 52, 52,
                52, 52, 52, 53, 53, 53, 53, 53, 53, 53, 54, 54, 54, 54, 54, 54, 55, 55, 56, 56, 56,
                56, 56, 56, 56, 56, 56, 57, 57, 57, 57, 57, 58, 59, 59, 59, 59, 59, 59, 60, 60, 60,
                60, 60, 60, 60, 60, 61, 61, 61, 61, 61, 61, 61, 61, 61, 61, 62, 62, 63, 63, 63, 63,
                63, 64, 65, 65, 65, 65, 65, 65, 65, 65, 65, 66, 66, 66, 66, 66, 66, 66, 66, 66, 67,
                67, 67, 67, 68, 68, 68, 68, 68, 68, 69, 69, 69, 69, 69, 69, 70, 70, 70, 70, 71, 72,
                72, 72, 72, 73, 73, 73, 73, 73, 73, 73, 73, 74, 74, 75, 75, 75, 75, 75, 75, 75, 75,
                75, 76, 76, 76, 76, 76, 76, 76, 76, 77, 77, 77, 77, 77, 77, 78, 78, 78, 78, 78, 79,
                79, 80, 80, 80, 80, 80, 80, 80, 81, 81, 81, 81, 81, 82, 82, 82, 82, 82, 82, 83, 83,
                83, 83, 83, 84, 84, 85, 85, 85, 85, 85, 85, 85, 86, 86, 87, 87, 87, 87, 87, 88, 88,
                88, 88, 88, 89, 89, 89, 89, 90, 90, 90, 90, 91, 91, 91, 91, 91, 91, 91, 92, 92, 92,
                92, 93, 93, 93, 93, 93, 94, 94, 94, 94, 95, 95, 95, 95, 95, 96, 96, 96, 97, 97, 97,
                98, 98, 98, 99, 99,
            ],
            "500 numbers between 0 and 100",
        ),
    ]
}
