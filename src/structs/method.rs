#[derive(Debug)]
struct Rectagle {
    width: i32,
    height: i32,
}

impl Rectagle {
    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn change_width(&mut self, w: i32) {
        self.width = w;
    }

    fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }
}

pub fn main() {
    let mut square = Rectagle::new(20, 20);

    let area = square.area();
    square.change_width(400);

    println!(
        "{square:?}, the area is {area}, {} is new area",
        square.area()
    )
}
