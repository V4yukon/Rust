#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

impl point {
    fn play(&self) {
        println!("I'm a method of Point.");
    }
}

fn main() {
    let boxed: Box<Point> = Box::new(Point{x: 10, y: 30});

    boxed.play();

    let y = &boxed;

    y.play();

    println!(("{:?}", y);
             }
