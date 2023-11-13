//trait
//
#[derive(Default)]
#[derive(Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    fn new(r: u8, g: u8, b: u8) -> Self {
        Color {
            r,
            g,
            b,
        }
    }
}


impl Color {
    fn red(r: u8) -> Self {
        Color {
            r,
            ..Color::default()
        }
    }
    fn green(g: u8) -> Self {
        Color {
            g,
            ..Color::default()
        }
    }
    fn blue(b: u8) -> Self {
        Color {
            b,
            ..Color::default()
        }
    }
}
fn main() {
    let a = Color{r: 8, g: 9, b: 10};
    println!("{:?}", a);
}
