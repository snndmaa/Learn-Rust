// Strings are similar to vectors since they are stored as a vector of bytes. The difference is that strings are guaranteed to always be a valid UTF 8 sequence
fn main() {
    let name = String::from("Tyler");
    let course = "Rust".to_string();
    let new_name = name.replace("Tyler", "Ty");

    println!("{}", name);
    println!("{}", course);
    println!("{}", new_name);

    // &str = "string slice or stir"
    // A string slice does not allocate memory on the heap where a string does
    let str1 = "hello"; // &str
    // println!("{}", str1.bogus()); // Throws an error, but in error type is &str not string

    let str2 = str1.to_string();
    let str3 = &str2;

    println!("{}", str1);
    println!("{}", str2);
    println!("{}", str3);

    // Comparing Strings - == !=
    println!("{}", "ONE".to_lowercase() == "one");

    // String Literal (Not valid utf 8)
    let rust = "\x52\x75\x73\x74";
    println!("{}", rust);
}
