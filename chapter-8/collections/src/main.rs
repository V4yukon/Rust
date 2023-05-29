fn main() {

    let m = String::from("lanmeizhenahochi");
    let n = vec![1, 3, 7];
    let v =vec![4, 5, 8, 9, 10];

    let mut c:Vec<i32> = Vec::new();

    c.push(3);
    c.push(7);
    c.push(8);

    let third: &i32 = &v[3];

    println!("the third = {third}");

    let third: Option<&i32> = v.get(30);

    match third{
        Some(third) => println!("the thied = {third}"),
        None => println!("there is no third"),
    }

    println!("the c is {:?}", c);

    let mut vm = vec![1, 4, 8, 10, 15];

    let first = &vm[2];
    
    println!("{first}");
    vm.push(3);

    println!("wow ,{:?}", vm);

    let mut vm1 = vec![4, 50, 100];

    for i in &mut vm1{
        *i += 50;
    }

    println!("changed {:?}", vm1);


    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }


    let row = vec![
        SpreadSheetCell::Int(64),
        SpreadSheetCell::Float(6.4),
        SpreadSheetCell::Text(String::from("hello world")),
    ];

   // println!("{:?}", row);
}
