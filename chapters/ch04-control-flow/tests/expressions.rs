use ch04_control_flow::*;

#[test]
fn grade_each_band() {
    assert_eq!(grade(95), 'A');
    assert_eq!(grade(85), 'B');
    assert_eq!(grade(75), 'C');
    assert_eq!(grade(65), 'D');
    assert_eq!(grade(30), 'F');
}

#[test]
fn grade_boundaries() {
    assert_eq!(grade(100), 'A');
    assert_eq!(grade(90), 'A');
    assert_eq!(grade(89), 'B');
    assert_eq!(grade(80), 'B');
    assert_eq!(grade(79), 'C');
    assert_eq!(grade(70), 'C');
    assert_eq!(grade(69), 'D');
    assert_eq!(grade(60), 'D');
    assert_eq!(grade(59), 'F');
    assert_eq!(grade(0), 'F');
}

#[test]
fn leap_year_ordinary_years() {
    assert!(is_leap_year(2024));
    assert!(is_leap_year(1996));
    assert!(!is_leap_year(2023));
    assert!(!is_leap_year(2025));
}

#[test]
fn leap_year_centuries() {
    assert!(!is_leap_year(1900)); // divisible by 100 but not by 400
    assert!(!is_leap_year(2100));
    assert!(is_leap_year(2000)); // divisible by 400
    assert!(is_leap_year(1600));
}

#[test]
fn median3_every_order() {
    // All six orderings of 1, 2, 3.
    assert_eq!(median3(1, 2, 3), 2);
    assert_eq!(median3(1, 3, 2), 2);
    assert_eq!(median3(2, 1, 3), 2);
    assert_eq!(median3(2, 3, 1), 2);
    assert_eq!(median3(3, 1, 2), 2);
    assert_eq!(median3(3, 2, 1), 2);
}

#[test]
fn median3_with_ties_and_negatives() {
    assert_eq!(median3(5, 5, 1), 5);
    assert_eq!(median3(1, 5, 5), 5);
    assert_eq!(median3(5, 1, 5), 5);
    assert_eq!(median3(1, 1, 5), 1);
    assert_eq!(median3(7, 7, 7), 7);
    assert_eq!(median3(-10, 0, -3), -3);
    assert_eq!(median3(i32::MIN, i32::MAX, 0), 0);
}
