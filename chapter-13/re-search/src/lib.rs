use std::env;


struct Config{
    query: String,
    file_path: String,
    ignore_case: bool,
}


impl Config{
    pub fn build(mut args: impl Iterator<Item = String> ) -> Result<Config, &'static str> {
        

        args.next();



        let query = match args.next(){
            Some(args) => args,
            None => return Err("Did't get a query string"),
        };


        let file_path = match args.next() {
            Some(args) => args,
            None => return Err("no file path"),
        };


        let ignore_case = env::var("IGNORE_CASS").is_ok();


        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}


fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {

    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
