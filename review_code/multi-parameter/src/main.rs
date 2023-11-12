struct typeA;
struct typeB;
struct typeC;


trait TraitA{}

impl TraitA for typeA{}
impl TraitA for typeB{}
impl TraitA for typeC{}


fn main() {
    fn doit(i: i32) -> impl TraitA {
        let a = typeA;
        a
    }

    fn dodo(i: i32) -> Box<dyn TraitA> {
        if i == 3 {
            let a = typeA;
            Box::new(a)
        } else if i  < 3{
            let b = typeB;
            Box::new(b)
        } else {
            let c = typeC;
            Box::new(c)
        }
    }
}
