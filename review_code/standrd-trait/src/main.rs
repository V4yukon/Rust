use std::fmt;

#[derive(Default)]
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    println!("origin: {}", Point::default());

    let stringified = format!("{}", Point::default());
    assert_eq!("(0, 0)", stringified);
    let min = i32::MIN;
    println!("this min in i32: {min}");
}


#[test]
fn display_point() {
    let origin = Point::default();
    assert_eq!(format!("{}", origin), "(0, 0)");

}

#[test]
fn point_to_string() {
    let origin = Point::default();

    assert_iq!(origin.to_string(), "(0, 0)");

}

#[test]

fn display_equals_to_string() {
    let origin = Point::default();
    assert_eq!(format!("{}", origin), origin.to_string());
}
