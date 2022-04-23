// You have two friends who are speaking Pig Latin to each other. Pig Latin is the same
// words in the same order except that you take the first letter of each word and put
// it to the end, then you add 'ay' to the end of that. ("road" = "oadray")
//
// Task:
// Your task is to take a senctence in English and turn it into the same
// sentence in Pig Latin!
//
// Input format:
// A string of the sentence in English that you seen to translate into Pig Latin.
// (No punctuation or capitalization)
//
// Output format
// A string of the same sentence in Pig Latin.

#[allow(dead_code)]
fn pig_latin(param: &str) -> String {
    param
        .split_whitespace()
        .map(|w| {
            // pig latinise a single word
            let mut pigl = String::from("");
            pigl.push_str(&(w.chars().skip(1).collect::<String>()));
            pigl.push(w.chars().nth(0).unwrap());
            pigl.push_str("ay");
            pigl
        })
        .collect::<Vec<String>>()
        .join(" ")
}

#[test]
pub fn pig_latin_test() {
    assert_eq!("ogay veroay heretay", pig_latin("go over there"));
    assert_eq!("allysay nowskay estbay", pig_latin("sally knows best"));
}