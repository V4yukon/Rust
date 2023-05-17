enum Coin{
    Penny,
    Nikcle,
    Dime,
    Quatar,
}


fn main(){
    variety_coin(Coin::Penny);
    println!("hello,world!");


    let five = Some(5);

    let sixe = comp_function(five);

    let none = comp_function(None);



    println!("five:{:?},six:{:?},NONE:{:?}",five,sixe,none); 
}


fn variety_coin(coin: Coin) -> u8{
    match coin{
        Coin::Penny => {
            println!("good");
            1
        },

        Coin::Nikcle => 3,
        Coin::Dime => 5,
        Coin::Quatar => 10,
    }
}
fn comp_function(som: Option<i32>) -> Option<i32>{
    match som {
        None => None,
        Some(i) => Some(i+1),
    }
}
