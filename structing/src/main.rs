struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuple Struct
struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

// Unit-like struct
struct _AlwaysEqual;

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("username"),
        email: String::from("username@email.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("another@email.com");

    let _user2 = build_user("username@email.com".to_string(), "username".to_string());

    let _user3 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("username@email.com"),
        sign_in_count: user1.sign_in_count,
    };

    let _user4 = User {
        email: String::from("username@email.com"),
        .._user3
    };

    let _black = Color(0, 0, 0);

    let _origin = Point(0, 0, 0);

    let _subject = _AlwaysEqual;
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
