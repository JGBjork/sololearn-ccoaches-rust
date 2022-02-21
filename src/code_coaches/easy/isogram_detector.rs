// An isogram is a word that has no repeating letters, whether they are consecutive or
// non-consecutive. your job is to find a way to detect if a word is an isogram.
//
// Task:
// Write a program that takes in a string as input, detects if the string is an isogram and outputs
// true of false based on the result.
//
// Input format:
// A string containing one word.
//
// Output format:
// A string: true or false

fn is_isogram(word: &str) -> bool {
    let mut s_vec: Vec<char> = word.chars().collect();
    s_vec[..].sort();
    for win in s_vec[..].windows(2) {
        if win[0] == win[1] { return false; }
    }
    true
}

#[allow(dead_code)]
pub fn isogram_detector(param: &str) -> String {
    format!("{:?}", is_isogram(param))
}

#[test]
pub fn isogram_detector_test() {
    assert_eq!(isogram_detector("come"), "true");
    assert_eq!(isogram_detector("cerebral"), "false");
}
