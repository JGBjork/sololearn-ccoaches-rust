// If a sentence flow, the first letter of each word will be the same to the last letter of the
// previous word.
//
// Task:
// Write a program that takes in a string that contains a sentence, checks if the first letter of
// each word is the same as thelast letter of the previous word. If the condition is met, output
// true, if not, output false. Casing does not matter.
//
// Input format:
// A string containing a sentence of words.
//
// Output format:
// A string: true or false.

#[allow(dead_code)]
pub fn flowing_words(params: &str) -> String {
    let sentence = String::from(params)[..].to_lowercase();
    let chunks = sentence[..].split(" ").collect::<Vec<&str>>();
    let is_flowing = chunks.windows(2).map( |w| {
        match w {
            [left, right] => {
                left.chars().last() == right.chars().next()
            },
            _ => false
        }
    }).reduce(|x,y| x & y).unwrap();
    format!("{}", is_flowing)
}

#[test]
pub fn flowing_words_test() {
    assert_eq!(flowing_words("compatable koalas"), "false");
    assert_eq!(flowing_words("TemporarY yield doesnT Turn"), "true");
}
