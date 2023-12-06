struct Url<'a> {
    protocol: &'a str,
    host: &'a str,
    path: &'a str,
    query: &'a str,
    fragment: &'a str,
}

fn foo() -> {
    let a = String::from("abs");

    {
        let b = String::from("def");
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else { 
        y
    }
}


fn main() {
    let s1 = String::from("logn string is long");
    let result;
    {
        let s2 = String::from("xyz");
        result = longest(s1.as_str(), s2.as_str());
    }

    println!("The longest string is {}", result);
}
