struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.height * self.width
    }
}

fn main() {
    let rect = Rect {
        width: 30,
        height: 15,
    };

    println!("The area of rectangle is {}", rect.area());
}