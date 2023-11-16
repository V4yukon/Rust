fn foo() -> Box<String> {
    let i = "abc".to_string();
    Box::new(i)
}
fn fpp() -> Box<i32> {
    let s = 30;
    Box::new(s)
}

#[derive(Debug)]
struct Point {
    x: u32, 
    y: u32,
}

impl Point {
    fn play(&self) {
        println!("I'm a method of Point");
    }
}


fn fqq() -> Box<Point> {
    let p = Point {x: 10, y: 20};
    Box::new(p)
}

fn main() {
    let _i = foo();
    let _s = fpp();

    println!("{:?}", _i);
    println!("{:?}", _s);
    

    let boxed: Box<Point> = Box::new(Point{x: 10, y: 20});
    boxed.play();
    println!("{:?}", boxed);


    
}
