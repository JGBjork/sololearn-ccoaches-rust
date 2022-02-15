// You have a bowl on your counter with an even
// number of pieces of fruit in it. Half of them
// are bananas, and the other half are apples.
// You need three apples to make a pie.
//
// Task:
// Your task is to evaluate the total number of pies
// that you can make with the apples that are in your
// bowl given the total amount of fruit in the bowl.
//
// Input format:
// An integer that represents the total amount of fruit
// in the bowl.
//
// Output format:
// An integer representing the total number of whole
// apple pies that you can make.

#[allow(dead_code)]
pub fn fruit_bowl(params: &str) -> String {
    match params.parse::<u32>() {
        Ok(n) => format!("{}", n/6),
        Err(_) => String::from("Error")
    }
}

#[test]
pub fn fruit_bowl_test() {
    assert_eq!(fruit_bowl("4"), "0");
    assert_eq!(fruit_bowl("12"), "2");
}
