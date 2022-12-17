use std::collections::HashMap;
fn main() {
    // let mut v: Vec<i32> = Vec::new();
    // let mut v = vec![1, 2, 3];

    // let first = &v[0];

    // v.push(6);

    // println!("The first element is: {first}");

    // let hello = "Здравствуйте";
    // let answer = &hello[0];

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}
