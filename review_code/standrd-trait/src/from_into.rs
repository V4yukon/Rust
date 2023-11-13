struct Point {
    x: i32,
    y: i32,
}


impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Point {x, y}
    }
}

impl From<[i32; 2]> for Point {
    fn from([x, y]): [i32; 2]) -> Self {
        Point {x, y}
    }
}

fn example() {
    let origin = Point::from((0, 0));
    let origin = Point::from([0, 0]);

    let origin: Point = (0, 0).into();
    let origin: Point = [0, 0].into();
}
