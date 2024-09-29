// References allow us to make a reference to a value without taking ownership of it. In other terms we are borrowing the value.
// There are 2 types of references:
//      - Shared References: These allow you to read to but not modify whatever is being referenced to. You can have as many as you desire.
//      - Mutable References: These allow you to both read and modify the value. You cannot have more than one active at the same tim to a specific value.
fn main() {
    let mut s = String::from("Hello");
    println!("{}", s);   // s prints because print_string function uses a shared reference.
    change_string(&mut s);  // Argument is a mutable reference
    println!("{}", s);   // We don't get borrow error here
}

fn change_string (some_string: &mut String) {   // Function changes the variable without taking ownership because of reference
    some_string.push_str(", world!");
}

fn _print_string (some_string: &String) {
    println!("{}", some_string);
}