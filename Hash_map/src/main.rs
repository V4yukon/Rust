use std::collections::HashMap;
fn main() {
    let mut lanmei = HashMap::new();

    lanmei.entry(String::from("blue")).or_insert(0);

    lanmei.entry(String::from("green")).or_insert(0);

    //let count = lanmei.entry(String::from("blue")).or_insert(1);

    //println!("{count}");
    //
    let stat = "hello world wondful world";


    let mut map = HashMap::new();

    for word in stat.split_whitespace(){

        let count = map.entry(word).or_insert(0);
        *count +=1;
    }
    
    println!("-------------------------------split-----------------------------");
    println!("this map will be: {:?}", map);
    println!("this map will be: {:?}", map);
    println!("-------------------------------split-----------------------------");
}

