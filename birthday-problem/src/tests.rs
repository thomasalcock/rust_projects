#![cfg(test)]

use crate::utils::{has_duplicates, contains_duplicates, simulate_birthdays};
use rand::rngs::ThreadRng;

#[test]
fn test_has_duplicates() {
    let mut x: Vec<i32> = vec![1, 2, 2, 3];
    let y: Vec<i32> = vec![1, 2, 3];
    let result: bool = has_duplicates(&mut x);
    assert_eq!(result, true);
    assert_eq!(x, y);
}

#[test]
fn test_contains_duplicates() {
    let immutable_x: Vec<i32> = vec![1, 2, 2, 3];
    let immutable_y: Vec<i32> = vec![1, 2, 2, 3];
    let result: bool = contains_duplicates(&immutable_x);
    assert_eq!(result, true);
    assert_eq!(immutable_x, immutable_y);
}

#[test]
fn test_simulate_birthdays() {
    let mut rng: ThreadRng = rand::thread_rng();
    let mut birthdays = simulate_birthdays(15, &mut rng);
    birthdays.sort();
    assert_eq!(birthdays.len(), 15);
    assert!(birthdays.last().copied().unwrap() < 365);
}
