// You have a box of popsicles and you want to give them all away to a group of brothers and
// sisters. If you have enough left in the box to give them each and even amount you should go for
// it! If not, they will fight over them, and you should eat them yourself.
//
// Task:
// Given the number of siblings that you are giving popsicles to, determine if you can give them
// each an even amount or if you shouldn't mention the popsicles at all.
//
// Input format:
// Two integer values, the first one represents the number of siblings, and the second one
// represents the number of popsicles that you have left in the box.
//
// Output format:
// A string that says 'give away' if you are giving them away, or 'eat them yourself' if you will
// be eating them yourself.

#[allow(dead_code)]
pub fn popsicles(params: &[&str]) -> String {
    match params {
        [siblings_s, popsicles_s] => {
            let siblings = siblings_s.parse::<u32>().unwrap();
            let popsicles = popsicles_s.parse::<u32>().unwrap();
            
            if popsicles % siblings == 0 { String::from("give away") }
            else { String::from("eat them yourself") }
        }
        _ => String::from("Error")
    }
}

#[test]
pub fn popsicles_test() {
    assert_eq!(popsicles(&["2", "5"]), "eat them yourself");
    assert_eq!(popsicles(&["10", "20"]), "give away");
}
