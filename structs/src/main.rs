struct User {
    email: String,
    username: String,
    active: bool,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let user_a = User {
        email: String::from("userA@gmail.com"),
        username: String::from("userA123"),
        active: true,
        sign_in_count: 1,
    };
    println!("{} {} {} {}", user_a.email, user_a.username, user_a.active, user_a.sign_in_count);

    let user_b = User {
        email: String::from("userB@gmail.com"),
        ..user_a
    };
    println!("{} {} {} {}", user_b.email, user_b.username, user_b.active, user_b.sign_in_count);

    let user_c = build_user(
        String::from("haitest@gmail.com"),
        String::from("haitest123"),
    );
    println!("{} {} {} {}", user_c.email, user_c.username, user_c.active, user_c.sign_in_count);

    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    println!("{}", black.0);
    println!("{}", origin.0);
}
