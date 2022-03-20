// You aren't paying attention and you accidentally type a bunch of random letters on your
// keyboard. You want to know if you ever typed the same letter twice, of if they are all unique
// letters.
//
// Task:
// If you are given a string of random letters, your task is to evaluate whether any letter is
// repeated in the string of if you only hit unique keys while you typing (sic!).
//
// Input Format:
// A string of random letter characters (no numbers or other buttons were pressed).
//
// Output Format:
// A string that says 'Deja Vu' if any letter is repeated in the input string, or a string that
// says 'Unique' if there are no repeats.
use std::collections::HashSet;

#[allow(dead_code)]
pub fn deja_vu(params: &str) -> String {
    let hs = params.chars().collect::<HashSet<char>>();

    if hs.len() == params.len() {
        "Unique".to_string()
    } else {
        "Deja Vu".to_string()
    }
}

#[test]
pub fn deja_vu_test() {
    assert_eq!(deja_vu("asd"), "Unique");
    assert_eq!(deja_vu("sammich"), "Deja Vu");
}
