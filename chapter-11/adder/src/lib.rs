pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]

    fn another() {
        panic!("oops, some error accured");
    }

    #[test]
    fn lager_can_hold_smaller() {
        let lager = Rectangle {
            width: 8, 
            height: 7,
        };
        let smaller = Rectangle {
            width: 5, 
            height: 1,
        };

        assert!(lager.can_hold(&smaller));
    }

    #[test]
    fn smaller_canot_hold_lager() {
        let lager = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5, 
            height: 1,
        };

        assert!(!smaller.can_hold(&lager));
    }
    #[test]
   // #[should_panic]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    //
    //using Result<T, E> in test
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
    
    #[test]
    fn this_test_will_pass() {
        let value = print_and_return10(10);
        assert_eq!(10, value);
    }
    #[test]
    #[ignore]
    fn this_test_will_fail() {
        let value = print_and_return10(5);
        assert_eq!(5, value);
    }
}

#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}


impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


////
////check panic with should_panic
////
////
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        //if value < 1 || value > 100 {
          //  panic!("Guess value must be between 1 and 100 , got {}.", value);
        //}
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
                );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
                );
        }

        Guess {value}
    }
}

/*
 *show print
 * */

pub fn print_and_return10(a: i32) -> i32 {
    println!("print {}", a);
    10
}


