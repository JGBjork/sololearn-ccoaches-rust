// You are robbing a bank, but you are not takinkg everything. You
// are looking for a specific item in the safety deposit boxes
// and you are going to drill into each one in order to find your item.
// Once you find your item you can make your escape, but how long will
// it take you to get that item?
//
// Task
// Determine the amount of time it will take you to find the item
// you are looking for if it takes you five minutes do drill into
// each box.
//
// Input format
// A string that represents the items in each box that will be drilled
// in order (items are separated by a comma), and secondly, a string
// of which item you are looking for.
//
// Output format
// An integer of the amount of time it will take for you to
// find your item.

fn safety_deposit_boxes(params: &[&str]) -> u32 {
    match params {
        &[items, needle] => {
            let (time, _) = items
                .split(',')
                .enumerate()
                .filter(|(_, straw)| *straw == needle)
                .nth(0).unwrap();
            (1 + time as u32) * 5
        }
        _ => 0
    }
}

#[test]
pub fn safety_deposit_boxes_test() {
    assert_eq!(5, safety_deposit_boxes(&["gold,silver,jewels,cheese", "gold"]));
    assert_eq!(10, safety_deposit_boxes(&["gold,silver,jewels,cheese", "silver"]));
}