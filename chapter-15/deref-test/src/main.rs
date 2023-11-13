use std::ops::Deref;

// #[derive(Debug)]
// struct Point {
//     x: i32,
//     y: i32,
// }
//
//
// impl From<(i32, i32)> for Point {
//     fn from((x, y): (i32, i32)) -> Self {
//         Point {x, y}
//     }
// }
//
// impl From<[i32; 2]> for Point {
//     fn from([x, y]: [i32; 2]) -> Self {
//     Point {x, y}
//     }
// }
//
// fn main() {
//     let origin1 = Point::from((0, 0));
//     let origin2 = Point::from([0, 0]);
//
//     let origin3: Point = (0, 0).into();
//     let origin4: Point = [0, 0].into();
//
//     println!("{:?}", origin1);
//     println!("{:?}", origin2);
//     println!("{:?}", origin3);
//     println!("{:?}", origin4);
// }
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for  MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    //
    // let a: i32 = 5;
    // let b = Box::new(a);
    //
    // assert_eq!(5, a);
    // assert_eq!(5, b);
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    let a = 5;
    let b = MyBox::new(a);
    assert_eq!(5, a);
    assert_eq!(5, *(b.deref()));
}
