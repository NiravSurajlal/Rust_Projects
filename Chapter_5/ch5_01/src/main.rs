struct User{
	username: String,
	email: String,
	sign_in_count: u64,
	active: bool,
}
fn main() {
    let mut user1 = User{
        email: String::from("123@sdhfb.zo.com"),
        username: String::from("person"),
        active: true,
        sign_in_count: 1,
    };
    //mut to allow this
    user1.email = String::from("changed.com");

    //tuple type struct
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
