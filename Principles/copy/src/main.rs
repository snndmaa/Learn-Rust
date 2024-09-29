// Copy is an operation that is implemented on types that are already stored on the stack such as integer, boolean, float, char and tuple(if every value it contains implements copy). 
fn main() {
    let x = 1;
    let y = x;
    println!("x = {}, y = {}", x, y);
}