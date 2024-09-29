fn main() {
    let s  = String::from("takes"); // Create a variable with the string takes
    takes_ownership(s); // Give ownership to the function
    //println!("{}", s);  // Function takes ownership so error is thrown

    let val = 1;
    make_copy(val);

    let str1: String = give_ownership();
    
    let str3: String = take_and_give(str1);
    //println!("{}", str1);   // throws error because take and give assumes ownership of str1

    if (true) {
        let str4 = str3;
    } else {
        let str5 = str3;
    }

    let str5 = String::from("Tyler");
    let mut str6: String;
    
    loop {
        str6 = str5;    // Throws error because move is made upon first iteration of loop
    }
}

fn takes_ownership(s: String) {
    let strin = s;
    println!("{}", strin);
}

fn make_copy(one: i32) {
    let val1 = one;
    println!("{}", val1);
}

fn give_ownership() -> String {
    return "given".to_string();
}

fn take_and_give(str2: String) -> String {
    str2
}