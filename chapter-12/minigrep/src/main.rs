use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    //let config = parse_config(&args);
    //
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Program parse arguments: {err}");
        process::exit(1)
    });

   // let query = &args[1];

    //let file_path = &args[2];

    //println!("The query is: {}",config.query);

    //println!("The search file path is: {}", config.file_path);

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
   // let contents = fs::read_to_string(config.file_path)
     //   .expect("Should have been able to read the file");


    //println!("With text:\n {contents}");
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
    
}
/*
// extract logic function 

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text: \n {contents}");
    
    Ok(())
}

struct Config {
    query: String,
    file_path: String,
}

// use method to instead function
//
//
/*impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();
        Config{query, file_path}
    }
}
*/
/*
 * use Result<T, E> instead of panic! macro
 * 
 */

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Parameters not enough")
        } else {
            let query = args[1].clone();
            let file_path = args[2].clone();
            Ok(Config{ query, file_path})
        }
    }
}

/*
fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config{ query, file_path}
}
*/
*/
