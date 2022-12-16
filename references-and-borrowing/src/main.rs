// fn main() {
//     let s1 = String::from("hello");

//     let s1_len = calculate_length(&s1);

//     println!("Length of {} is {}", s1, s1_len)
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// Try to change something borrowed
// error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
//   --> src/main.rs:21:5
//    |
// 20 | fn change(some_string: &String) {
//    |                        ------- help: consider changing this to be a mutable reference: `&mut std::string::String`
// 21 |     some_string.push_str(", world");
//    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable
// fn main()  {
//     let s = String::from("hello");

//     change(&s)
// }

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }
// => to fix this
fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("{}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
