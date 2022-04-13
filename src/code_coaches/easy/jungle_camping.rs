// You are camping alone out in the jungle and you hear some animals in the dark nearby.
// Based on the noise they make, determine which animals they are.
//
// Task:
// You are given the noises made by different animals that you can hear in the dark, evaluate each
// noise to determine which animal it belongs to. Lions say 'Grr', Tigers say 'Rawr',
// Snakes say 'Ssss', and Birds say 'Chirp'
//
// Input format:
// A string that represents the noises that you hear with a space between them
//
// Output format:
// A string that includes each animal that you hear with a space after each one.
// (Animals can repeat)

#[allow(dead_code)]
pub fn jungle_camping(params: &str) -> String {
    params.split_whitespace().map(
        |sound| {
            match sound {
                "Grr" => "Lion",
                "Ssss" => "Snake",
                "Chirp" => "Bird",
                "Rawr" => "Tiger",
                _ => "Unknown"
            }
        }
    ).collect::<Vec<&str>>().join(" ")
}

#[test]
pub fn jungle_camping_test() {
    assert_eq!("Lion Lion", jungle_camping("Grr Grr"));
    assert_eq!("Snake", jungle_camping("Ssss"));
}