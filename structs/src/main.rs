fn main() {
    // define a new struct type
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let user1 = User {
        email: String::from("user@example.com"),
        username: String::from("someone_username"),
        active: true,
        sign_in_count: 1,
    };
}
