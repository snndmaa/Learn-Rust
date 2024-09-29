// Compound types can group multiple values into one type.

fn main() {
    let tup = (500, "hi", true); // By default a tuple is immutable however you can add the mut keyword to make it mutable
    println!("{}", tup.2);

    let (x, _y, _z) = tup;
    println!("{}", x);
}
