#[derive(Debug, Clone)]

struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn play(&self) {
        println!("I'm an method of Point");
    }
}

fn main() {
    let mut boxed: Box<Point> = Box::new(Point{x: 10, y: 20});
    let mut another_boxed = boxed.cloen();

    *another_boxed = Point{x: 100, y: 200};

    println!("{:?}", boxed);
    println!("{:?}", another_boxed);
}
