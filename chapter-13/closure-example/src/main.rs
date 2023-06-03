use std::thread;

#[derive(Debug)]

struct Rectangle{
    width: u32,
    height: u32,
}




fn main() {
    //let mut list = vec![3, 7, 8, 9];

    //println!("before borrow {:?}", list);
    //let mut lanmei = || list.push(7);

    //lanmei();
    //println!("changed {:?}", list);

    //thread::spawn( || println!("From thread {:?}", list))
      //  .join()
       // .unwrap();
    
    let mut list = [
        Rectangle{ width: 4, height: 7},
        Rectangle{ width: 3, height: 5},
        Rectangle{ width: 7, height: 2},
    ];
    

    let mut sort_operation = 0;

    let value = String::from("by key called");

    list.sort_by_key(|r| {
                     //sort_operation.push(value);
                     sort_operation += 1;
                     r.height});

    println!("{:#?}   time_counter: {sort_operation}", list);


}



