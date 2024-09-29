fn main() {
    let r;

    {
        let x = 5;
        r = &x;
    }
    // x is dropped from memory

    //println!("{}", r);  // this outputs an error because the borrowed value r is referencing dies after the block/scope is executed. Therefore you get the error that the borrowed value does not live long enough.
    // 'a - Lifetime Setup
    // &i32
    // &'a i32 - Explicit lifetime setup of an i32
    // &'a mut i32
}

fn example<'a>(x: &'a str) ->