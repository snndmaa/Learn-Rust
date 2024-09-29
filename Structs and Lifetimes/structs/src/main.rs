// Structs which are short for structures, let you name and package together related values into a single value so you can deal with them as a single unit.
// Rust has 3 types of Structs:
//      - A named field: Gives a name to each component
//      - Tuple like: Identifies them by the order in which they appear
//      - Unit like: Has no components at all
struct User {   // Name field struct
    active: bool,
    username: String,
    sign_in_count: u32,
}

struct Coordinates(i32, i32, i32);  // Tuple-like struct

struct UnitStruct;  //Unit-like struct. The 1..5, the .. is shorthand for range {start: 1, end: 5}

fn main() {
    let user1 = User{active: true, username: String::from("Byron"), sign_in_count: 0};
    println!("{}", user1.username);

    let user2 = build_user(String::from("Tyler"));
    println!("{}", user2.username);

    let cords = Coordinates(1, 2, 3);
}

fn build_user(username: String) -> User {
    User{
        username,
        active: true,
        sign_in_count: 1,
    }
}