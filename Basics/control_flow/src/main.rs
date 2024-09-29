fn main() {
    let one = 1;

    if one > 10 {
        println!("True");
    } else if one == 1 {
        println!("Equal");
    } else {
        println!("false");
    }

    // loop_demo();
    // while_loop_demo();
    // for_loop_demo();
    for_loop_extra();
}

fn _loop_demo() {
    let mut num: u8 = 0;
    'counter: loop {
        println!("Count: {}", num);
        let mut decrease: i8 = 5;
        loop {
            println!("Decreasing: {}", decrease);
            if decrease == 4 {
                break;
            } else if num == 2 {
                break 'counter;
            }

            decrease -= 1;
        }
        num += 1;
    }
}

fn _while_loop_demo() {
    let mut num = 0;
    while num < 5 {
        println!("Num: {}", num);
        num += 1;
    }
}

fn _for_loop_demo() {
    let vec: Vec<i8> = (0..10).collect();

    for element in vec {
        println!("{}", element);
    }
}

fn for_loop_extra() {
    for number in (0..4).rev() {
        println!("{}", number)
    }
    println!("LIFTOFF!!!")
}