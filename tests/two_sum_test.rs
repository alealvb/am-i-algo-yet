use am_i_algo_yet::two_sum::TwoSum;

#[test]
fn test_two_sum() {
    let test_cases = [
        (vec![2, 7, 11, 15], 9, vec![0, 1]),
        (vec![2, 7, 11, 15], 18, vec![1, 2]),
        (vec![3, 2, 4], 6, vec![1, 2]),
        (vec![3, 3], 6, vec![0, 1]),
        (vec![-1, 0], -1, vec![0, 1]),
        (vec![-1, 0], 0, vec![]),
        (vec![4, 2, -2, -3, 1], 0, vec![1, 2]),
        (vec![4, 2, -2, -3, 1], -5, vec![2, 3]),
        (vec![9, 5, 7, 3, 2, 11], 10, vec![2, 3]),
        (vec![9, 5, 7, 3, 2, 11], 11, vec![0, 4]),
        (vec![100, 43, 12, 445], 457, vec![2, 3]),
        (vec![100, 43, 12, 445], 2, vec![]),
        (vec![-100, 43, 12, 445], -100, vec![]),
        (
            vec![
                2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
                25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45,
                46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66,
                67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87,
                88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100,
            ],
            500,
            vec![],
        ),
        (
            vec![
                2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
                25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45,
                46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66,
                67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87,
                88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100,
            ],
            199,
            vec![97, 98],
        ),
    ];

    for (nums, target, expected) in test_cases.iter() {
        let mut result = TwoSum::two_sum(&nums, target);
        result.sort();

        assert_eq!(
            result,
            expected.clone(),
            "\nnums: {:?}\n target: {}",
            nums,
            target
        );
    }
}