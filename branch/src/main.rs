//use std::ops::Range;
use std::io;
fn main() {
    let mut count = 0;

    'counting_up: loop{
        println!("count is {count}");

        let mut remaining = 10;


        loop{
            println!("remaining is {remaining}");

            if remaining == 9  {
                break;
            }

            if count == 2{
                break 'counting_up;
            }
            remaining -= 1;

        }

        count += 1;
    }
    println!("End count = {count}");


    //another_funciton();

    for_function();

    let mut muss = String::new();

    io::stdin()
        .read_line(&mut muss)
        .expect("Failed to read line");

    let muss : u32  = muss.trim().parse().expect("Please input a number");
    while muss < 5 {  
        println!("muss is {muss}");
        }
} 
//fn another_funciton(){
  //  for number in (1..5).rev{
    //    println!("{number}!");
   // }
//}

fn for_function(){
    let a = [3,5,7,10,11];

    for element in a{
        println!("list a is {element}");
    }
}


