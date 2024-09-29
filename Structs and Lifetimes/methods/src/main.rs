struct Square {
    width: u32,
    height: u32,
}

impl Square {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> u32 {
        self.width
    }

    fn change_width(&mut self, new_width: u32) {
        self.width = new_width
    }
}
fn main() {
    let mut sq = Square {width: 5, height: 5};
    println!("{}", sq.area());
    sq.change_width(2);
    println!("{}", sq.width());
}
