use std::fs;
use std::collections::HashMap;
fn main() {
   let open_my_file = fs::read_to_string("hello.txt").expect("failed to read file");

   let mut map = HashMap::new();

   for word in open_my_file.split_whitespace(){
       let count = map.entry(word).or_insert(0);
       *count +=1;
   }
   println!("{:?}", map);
}

