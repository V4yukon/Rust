//struct Point<T> {
  //  x: T,
   // y: T,
//}

//impl<T> Point<T> {
  //  fn x(&self) ->&T{
    //    &self.x
    //}
//}

//fn main() {

  //  let num1 = Point{x: 4, y: 5};
   // println!("Hello, world!{}",num1.x());
//}
//
//




struct Point<x1, y1> {
    x: x1,
    y: y1,
}

impl<x1, y1> Point<x1, y1> {
    fn mixup<x2, y2> (self, other: Point<x2, y2>) -> Point<x1, y2> {
        Point{
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point {x: 5, y: 10.4};
    let p2 = Point {x: "hello", y: 'c'};
    let p3 = p1.mixup(p2);


    println!("p3.x={}, p3.y={}", p3.x, p3.y);
}
