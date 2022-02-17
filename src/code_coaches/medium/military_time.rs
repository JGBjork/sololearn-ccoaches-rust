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
    let (time,time_of_day) = params.split_once(' ').unwrap();
    let (hour, minutes) = time.split_once(':').unwrap();
    match (hour.parse::<u32>(), minutes.parse::<u32>()) {
        (Ok(mut h), Ok(m)) => {
            h %= 12;
            if time_of_day == "PM" { h += 12; }
            format!("{:02}:{:02}", h, m)
        },
        _ => String::from("Error")
    }
}

#[test]
pub fn military_time_test() {
    assert_eq!(military_time("11:00 AM"), "11:00");
    assert_eq!(military_time("11:00 PM"), "23:00");
    assert_eq!(military_time("12:00 AM"), "00:00");
    assert_eq!(military_time("12:00 PM"), "12:00");
    assert_eq!(military_time("1:05 AM"), "01:05");
    assert_eq!(military_time("1:05 PM"), "13:05");
}
