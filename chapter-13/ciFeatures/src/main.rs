#[warn(non_snake_case)]
#[derive(Debug, PartialEq, Copy, Clone)]

enum ShirtColor{
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}


impl Inventory {
    fn giveaway(&self, user_perference: Option<ShirtColor>) -> ShirtColor {
        user_perference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue +=1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }

    }
}

fn main() {

    let store = Inventory{
        shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Red],
    };

    let user_pre1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pre1);

    println!(
        "The user1 prefer {:?} give {:?}", user_pre1, giveaway1);

    let user_pre2 = None;
    let giveaway2 = store.giveaway(user_pre2);
    
    println!(
        "The user2 prefer {:?} give {:?}", user_pre2, giveaway2);
}
