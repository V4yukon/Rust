#[derive(Debug)]

struct Rectangle{
    width: u32,
    height: u32,
}
fn main() {
  // let width = 30;
   //let height = 50;
    let rectangle = Rectangle{
        width: 30,
        height: 50,
    };
   //println!("the area of rectangle is {}", area_function(width, height));
    // println!("Hello, world!");
    println!("The area of rectangle is {} which was refactored by struct ", area(&rectangle));
    println!("The rectangle is {:?}", rectangle);
}



//fn area_function(width: u32, height: u32) -> u32{
  //  width * height
//}


fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
