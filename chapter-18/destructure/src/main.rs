struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let p = Point{
        x: 0,
        y: 3,
    };
    /*
    let Point{x: a, y: b,} = p;

    assert_eq!(a, 0);
    assert_eq!(3, b);
    */

    match p {
        Point {x, y: 0} => println!("at the x axis {x}"),
        Point {x: 0, y} => println!("at the y axis {y}"),
        Point {x, y} => println!("{x}, {y}"),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => { println!("The QUit variatant have no data to destructure");
        }
        Message::Move {x, y} => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r,g,b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}",)
        }
    }
}
//shorthand the pattern 
//can ignore variables name 
