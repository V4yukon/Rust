use obtrait::{Button, Screen, Draw};

pub struct SelectBox {
    width: u32,
    height: u32,
    label: Vec<String>, 
}

impl Draw for SelectBox {
    fn draw(&self) {
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75, 
                height: 10,
                label: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("Ok"),
            }),
    ],
};

screen.run();
}

