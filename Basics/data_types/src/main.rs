fn main() {
    let x: i8 = -10;
    println!("{}", x);

    let _y: u8 = 10;

    let decimal = 02_55; // Also 255
    let hex = 0xff;
    let octal = 0o377;
    let binary = 0b1111_1111;

    println!("decimal: {}", decimal);
    println!("hex: {}", hex);
    println!("octal: {}", octal);
    println!("binary: {}", binary);

    let byte = b'A';
    println!("{}", byte);

    println!();

    let _a = 2.0; // f64 is default because on mordern cpu's this size is almost same speed as f32. Also gives better precision
    let _b: f32 = 1.0;
    
    let _c = true; // Rust's compiler infers the type
    let _d: bool = false;

    let e = 'e';
    println!("{}", e);
}
