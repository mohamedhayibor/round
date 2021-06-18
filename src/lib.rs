// round - rust library (crate)
// GNU licensed, license file can be found at the root of the repository
// Mohamed Hayibor - Copyright 2016

pub fn round(number: f64, rounding: i32) -> f64 {
  let scale: f64 = 10_f64.powi(rounding);
  (number * scale).round() / scale
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

// implementing round_up and round_down with same design pattern
pub fn round_up(number: f64, rounding: i32) -> f64 {
  let scale: f64 = 10_f64.powi(rounding);
  (number * scale).ceil() / scale
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

pub fn round_down(number: f64, rounding: i32) -> f64 {
  let scale: f64 = 10_f64.powi(rounding);
  (number * scale).floor() / scale
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

