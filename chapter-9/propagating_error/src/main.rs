
//use std::io::{self, read};
//use std::fs::File;

fn main() {
    let  m  = "hello world!";

    for c in m.chars() {

        if c == 'l'{
            println!("dao ci weizhi ba");
            break;
        }else{
            println!("{:?}", c);
            continue;
        }
    }
}
