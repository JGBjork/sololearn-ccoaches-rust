// As headmaster of the post office, sometimes people write zip codes that don't exist or zip codes
// that are not valid. You are tasked with making a system to check if the inputted zip code is a
// valid zip code.
//
// Task:
// Write a program that takes in a string representing a zip code. Output true or false if it is a
// valid zip code or not. A valid zip code is only numbers, must be 5 characters in length, and
// contain no spaces.
//
// Input format:
// A string containing a zip code.
//
// Output format:
// A string: true is (sic!) the input is a valid zip code, or false if it is not.

use regex::Regex;

#[allow(dead_code)]
pub fn zip_code_validator(params: &str) -> String {
    let validator = Regex::new(r"^\d{5}$").unwrap();
    format!("{:?}", validator.is_match(params))
}

#[test]
pub fn zip_code_validator_test() {
    assert_eq!(zip_code_validator("52452"), "true");
    assert_eq!(zip_code_validator("424"), "false");
}
