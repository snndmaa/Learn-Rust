// A clone takes the values from a variable we are referencing and copies them into a new variable. Doing this may be expensive especially for large data.
fn main() {
    let x = vec!["byron".to_string()];
    let y = x.clone();  //Deep copy of x
    let z = y.clone();

    println!("{:?}", x);
    println!("{:?}", y);
    println!("{:?}", z);
}
