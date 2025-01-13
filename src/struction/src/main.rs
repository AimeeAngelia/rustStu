struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
};

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    let build = build_user(String::from("love@you.com"), String::from("kawaii"));
    println!("{}\n{}",build.username, build.email);
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("anotherlove@you.com"),
        sign_in_count: user1.sign_in_count,
    };
    let user3 = User {
        email: String::from("NO@love.com"),
        ..user2
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        email,
        username,
        sign_in_count: 1,
    }
}

