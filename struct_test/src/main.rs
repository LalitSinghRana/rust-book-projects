struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        email: String::from("abc@xyz.com"),
        active: true,
        sign_in_count: 14,
        username: String::from("abc.123"),
    };

    user1.email = String::from("qwerty@gmail.com");

    let user2 = User {
        email: String::from("zxcv@ms.com"),
        ..user1
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
