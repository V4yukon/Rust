fn main() {
    println!("Hello, world!");
    let x = plus_function(5);
    println!("the result of plus_function is {x}");
    another_function();
}
fn another_function(){
    println!("This is another function!!!!");
}
fn plus_function(x:i32) -> i32 {
    x + 5
}
