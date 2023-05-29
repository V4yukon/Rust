fn main() {

    let s = String::from("hello");

    let len = refer_function(&s);

    println!("the string {} lenth is {}", s, len);



    println!("Hello, world!");
}


fn refer_function(s : &String) -> usize{
    s.len()
}
