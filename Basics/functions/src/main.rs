fn main() {
    print_phase("one");
    println!("{}", gcd(20, 4));
}

fn print_phase(x: &str) {
    println!("Print these arguments: {}", x);
}

fn gcd(mut a: u64, mut b: u64) -> u64 { // Greatest Common Denominator  
    while a != 0{
        if a < b {
            let c = a;
            a = b;
            b = c;
        }
        a = a % b;
    }
    b
}

fn _multiple_return_values(flag: bool) -> bool {
    if flag == true {
        true
    } else {
        false
    }
}