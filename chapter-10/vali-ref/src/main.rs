use std::fmt::Display;
fn main() {
    let string1 = String::from("abcd");
    let string2 = "zyz";

    let result = longest(string1.as_str(), string2);

    println!("the result:{}", result);

    let string3 = String::from("longest sentence will show");
    {
        let string4 = String::from("ok you are right");
        let result1 = longest(string3.as_str(), string4.as_str());
        println!("the result1 : {}", result1);
    }
}

//implement the function of longest

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    }else{
        y
    }
}

//lifetime in struct

struct ImportErt<'a> {
    part: &'a str,
}
//
//lifetime elision


fn verbos_summary<'a, T> (
    x: &'a str,
    y: &'a str,
    ann: T,
    ) -> &'a str 
    where
    T: Display,
{
    println!("The announcement is {}", ann);
    if x.len() > y.len() {
        x
    }else{
        y
    }
}
