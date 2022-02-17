// You want to convert the time from a 12 hour clock to a 24 hour clock. If you are given the time
// on a 12 hour clock, you should output the time as it would appear on a 24 hour clock.
//
// Task:
// Determine if the time you are given is AM or PM, then convert that clue to the way that it would
// appear on a 24 hour clock.
//
// Input format:
// A string that includes the time then a space and the indicator AM or PM.
//
// Output format:
// A string that inlcudes the time in a 24 hour format (XX:XX)

#[allow(dead_code)]
pub fn military_time(params: &str) -> String {
    String::from("not implemented yet")
}

#[test]
pub fn military_time_test() {
    assert_eq!(military_time("11:00 AM"), "11:00");
    assert_eq!(military_time("11:00 PM"), "23:00");
}
