// You go trick or treating with a friend and all but three of the houses that you visit are giving
// out candy. One house that you visit is giving out toothbrushes and two houses are giving out
// dollar bills.
//
// Task:
// Given the input of the total number of houses that you visited, what is the percentage chance
// that one random item pulled from your bag is a dollar bill?
//
// Input Format:
// An integer (>=3) representing the number of houses that you visited.
//
// Output Format:
// A percentage value rounded up to the nearest whole number.

#[allow(dead_code)]
pub fn halloween_candy(params: &str) -> String {
    match params.parse::<u32>() {
        Ok(n) => {
            let mut probability = 200.0 / (n as f64);
            probability = probability.ceil();
            format!("{}", probability as i32)
        },
        Err(_) => String::from("Error")
    }
}

#[test]
pub fn halloween_candy_test() {
    assert_eq!(halloween_candy("3"), "67");
    assert_eq!(halloween_candy("10"), "20");
}
