fn main() {
    let x = vec!["byron".to_string()];
    let y = x;  // Value of x is moved to y, therefore x no longer has it
    let z = y;  // Value of y is moved to z, therefore y no longer has it
    println!("{:?}", z);
}
