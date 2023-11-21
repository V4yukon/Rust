use std::env;

fn main() {
    let addr = env::args()
        .nth(1)
        .unwrap_or("127.0.0.1".to_string());

    println!("{}", addr);

}
