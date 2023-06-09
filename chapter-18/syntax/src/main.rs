fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);
    

     let m = 5;

    match m {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let l = 1;
    match l {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("everything"),
    }


    let k = 'c';
    match k {
        'a' ..= 'h' => println!("early AscII"),
        'i' ..= 'z' => println!("hahah,good"),
        _ => println!("anything else"),

    }

}
