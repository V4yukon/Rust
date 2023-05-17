//pub fn add(left: usize, right: usize) -> usize {
  //  left + right
//}

//#[cfg(test)]
//mod tests {
  //  use super::*;

    //#[test]
   // fn it_works() {
     //   let result = add(2, 2);
       // assert_eq!(result, 4);
   // }
//}
//
#[warn(dead_code)]
pub mod back_of_house {
    pub struct Breakfirst {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfirst {
        pub fn summer(toast: &str) -> Breakfirst {
            Breakfirst {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_resturant(){
    let mut meal = back_of_house::Breakfirst::summer("Rye");

    meal.toast = String::from("Wheat");


    println!("I'd like {} toast", meal.toast);
}
