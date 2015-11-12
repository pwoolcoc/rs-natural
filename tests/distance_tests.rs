extern crate natural;
extern crate num;

use natural::distance::jaro_winkler_distance;
use natural::distance::levenshtein_distance;
use num::Float;

#[test]
fn test_jaro_winkler() {
  f64_eq(jaro_winkler_distance("one", "one"), 1f32);
  f64_eq(jaro_winkler_distance("one", ""), 0f32);
  f64_eq(jaro_winkler_distance("", ""), 0f32);
  f64_eq(jaro_winkler_distance("", "one"), 0f32);
  f64_eq(jaro_winkler_distance("DWAYNE", "DUANE"), 0.822);
  f64_eq(jaro_winkler_distance("Martha", "Marhta"), 0.944);
  f64_eq(jaro_winkler_distance("dixon", "dicksonx"), 0.767);
}

#[test]
fn test_jaro_strings() {
  f64_eq(jaro_winkler_distance("one".to_string(), "one".to_string()), 1f32);
  f64_eq(jaro_winkler_distance("one", "one".to_string()), 1f32);
  f64_eq(jaro_winkler_distance("one".to_string(), "one"), 1f32);
  f64_eq(jaro_winkler_distance("one", "one"), 1f32);
}

fn f64_eq(a: f32, b: f32) {
  assert!((a - b).abs() < 0.01);
}

#[test]
fn test_lev_distance() {
  assert_eq!(levenshtein_distance("one", "one"), 0);
  assert_eq!(levenshtein_distance("one", ""), 3);
  assert_eq!(levenshtein_distance("", ""), 0);
  assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
}
