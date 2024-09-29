fn main() {
    let array: [u8; 3] = [1, 2, 3];
    println!("{}", array[0]);
    
    let mut array2: [u16; 2] = [300, 400];
    println!("{:?}", array2);
    array2[1] = 500;
    println!("{}", array2[1]);
}
