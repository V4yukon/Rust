use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();


    let query = &args[1];

    let file_path = &args[2];

    println!("The query is: {}", query);

    println!("The search file path is: {}", file_path);

    //dbg!(args);
    /*let length = &args.len().to_int();
    if length > 3 || length == 1 {
        println!("Please check the parametes , you dont input correct");
    } else {
        println!("good");
    }
    */


    // open file
    //
    //
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");


    println!("With text:\n {contents}");
    
}

