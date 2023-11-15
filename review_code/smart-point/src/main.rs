fn foo() -> Box<String> {
    let i = "abc".to_string();
    Box::new(i)
}
fn fpp() -> Box<i32> {
    let s = 30;
    Box::new(s)
}

fn main() {
    let _i = foo();
    let _s = fpp();

    println!("{:?}", _i);
    println!("{:?}", _s);
}
