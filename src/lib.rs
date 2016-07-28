// round - rust library (crate)
// GNU licensed, license file can be found at the root of the repository
// Mohamed Hayibor - Copyright 2016

pub fn round(number: f64, rounding: i32) -> f64 {
    match rounding {
        1        => (number * 10.).round() / 10.,
        2        => (number * 100.).round() / 100.,
        3        => (number * 1000.).round() / 1000.,
        4        => (number * 10000.).round() / 10000.,
        5        => (number * 100000.).round() / 100000.,
        6        => (number * 1000000.).round() / 1000000.,
        7        => (number * 10000000.).round() / 10000000.,
        8        => (number * 100000000.).round() / 100000000.,
        9        => (number * 1000000000.).round() / 1000000000.,
        10       => (number * 10000000000.).round() / 10000000000.,
        _        => (number * 100.).round() / 100.,
    }
}

#[test]
fn test_round_by2() {
    let test_n = round(8.9534, 2);
    assert_eq!(test_n, 8.95);
}

#[test]
fn test_round_by3() {
    let test_n = round(8.9536, 3);
    assert_eq!(test_n, 8.954);
}

#[test]
fn test_default_rounding() {
    let test_n = round(8.9536, -1);
    assert_eq!(test_n, 8.95);
}
