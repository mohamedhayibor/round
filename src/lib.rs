// round - rust library (crate)
// GNU licensed, license file can be found at the root of the repository
// Mohamed Hayibor - Copyright 2016

pub fn round(number: f64, rounding: i32) -> f64 {
    match rounding {
        0        => number.round(),
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
fn test_round_by0() {
    let test_n = round(8.9534, 0);
    assert_eq!(test_n, 9.0);
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

// implementing round_up and round_down with same design pattern
pub fn round_up(number: f64, rounding: i32) -> f64 {
    match rounding {
        1        => (number * 10.).ceil() / 10.,
        2        => (number * 100.).ceil() / 100.,
        3        => (number * 1000.).ceil() / 1000.,
        4        => (number * 10000.).ceil() / 10000.,
        5        => (number * 100000.).ceil() / 100000.,
        6        => (number * 1000000.).ceil() / 1000000.,
        7        => (number * 10000000.).ceil() / 10000000.,
        8        => (number * 100000000.).ceil() / 100000000.,
        9        => (number * 1000000000.).ceil() / 1000000000.,
        10       => (number * 10000000000.).ceil() / 10000000000.,
        _        => (number * 100.).ceil() / 100.,
    }
}

#[test]
fn test_round_up_by2() {
    let test_n = round_up(8.9534, 2);
    assert_eq!(test_n, 8.96);
}

#[test]
fn test_round_up_by3() {
    let test_n = round_up(8.9536, 3);
    assert_eq!(test_n, 8.954);
}

#[test]
fn test_default_round_up() {
    let test_n = round_up(8.9536, -1);
    assert_eq!(test_n, 8.96);
}

pub fn round_down(number: f64, rounding: i32) -> f64 {
    match rounding {
        1        => (number * 10.).floor() / 10.,
        2        => (number * 100.).floor() / 100.,
        3        => (number * 1000.).floor() / 1000.,
        4        => (number * 10000.).floor() / 10000.,
        5        => (number * 100000.).floor() / 100000.,
        6        => (number * 1000000.).floor() / 1000000.,
        7        => (number * 10000000.).floor() / 10000000.,
        8        => (number * 100000000.).floor() / 100000000.,
        9        => (number * 1000000000.).floor() / 1000000000.,
        10       => (number * 10000000000.).floor() / 10000000000.,
        _        => (number * 100.).floor() / 100.,
    }
}

#[test]
fn test_round_down_by2() {
    let test_n = round_down(8.9534, 2);
    assert_eq!(test_n, 8.95);
}

#[test]
fn test_round_down_by3() {
    let test_n = round_down(8.9536, 3);
    assert_eq!(test_n, 8.953);
}

#[test]
fn test_default_round_down() {
    let test_n = round_down(8.9536, -1);
    assert_eq!(test_n, 8.95);
}
