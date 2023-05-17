fn main() {
   let x = String::from("hello,world!");

   let y = x;

   println!("{y}");
   let a = (3,5,7);

   println!("Tuple {:?}", a);

   let b = a ;
   println!("Tuple {:?}", b);

    let m = String::from("hello");

    string_function(m);

    let n = 10;

    inter_function(n);

   // println!("{m}");

    println!("{n}");
  
   //println!("{x}");
    // println!("Hello, world!");
    //
    //
    //
    let mut s = String::from("hello");


    change(&mut s);






    //mutable reference 

    let r1 = &mut s;
    let r2 = &mut s;


    println!("{} {}", r1, r2);
}


fn inter_function(m : i32){
    println!("{m}");
}
fn string_function(n : String){
    println!("{n}");
}


fn move_string() -> String{
    let y = String::from("wut");

    y
}


fn change(some_string : &mut String) {
    some_string.push_str(",world!")
}
