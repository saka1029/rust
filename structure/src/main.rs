mod rect;

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anothermail@example.com");
    user1 = build_user(String::from("saka1029@gmail.com"),
        String::from("saka1029"));
    print_user(&user1);
    let user2 = User {
        email: String::from("user2@example.com"),
        username: String::from("user2"),
        ..user1
    };
    print_user(&user2);
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    #[derive(Debug)]
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black={:?}, origin={:?}", black, origin);
    println!("black=({},{},{}), origin=({},{},{})",
    black.0, black.1, black.2,
    origin.0, origin.1, origin.2);
    rect::main_rect();
}

fn print_user(user: &User) {
    println!("User({}, {}, {}, {})",
        user.username, user.email, user.active, user.sign_in_count);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
