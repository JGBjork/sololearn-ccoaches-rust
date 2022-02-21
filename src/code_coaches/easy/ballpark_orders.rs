// You and three friends go to a baseball game and you offer to go to the concessions stand for
// everyone. They each order one thing, and you do as well. Nachos and Pizza both cost $6.00. A
// Cheeseburger meal costs $10.00. Water is $4.00 and Coke is $5.00. Tax is 7%.
//
// Task:
// Determine the total cost of ordering four items from the concession stand. If one of your
// friend's (sic!) orders something that isn't on the menu, you will order a Coke for them instead.
//
// Input format:
// You are given a string of the four items that you've been asked to order that are separated by
// spaces.
//
// Output format:
// Youwill output a numberof the total cost of the food and drinks.

#[allow(dead_code)]
pub fn ballpark_orders(params: &str) -> String {
    let total: f64 = 
        params.split(" ")
        .map(|order| match order {
            "Nachos" | "Pizza" => 6.0,
            "Cheeseburger" => 10.0,
            "Water" => 4.0,
            _ => 5.0
         })
        .sum();
    format!("{:.2}", (total * 107.00) / 100.0)
}

#[test]
pub fn ballpark_orders_test() {
    assert_eq!(ballpark_orders("Water Water Water Water"), "17.12");
    assert_eq!(ballpark_orders("Cheeseburger Cheeseburger Coke Water"), "31.03");
}
