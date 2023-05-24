pub trait Summary{
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {}
    //fn summarize(&self) -> String {
      //  format!("{},by{} {}", self.headline, self.author, self.location)
   // }
//}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}:{}", self.username, self.content)
    }
}

pub fn notify(item: impl Summary) {
    println!("Breaking news {}", item.summarize());
}
//
//***********There two parameters have the same type************
//**********trait bound syntax************
//pub fn notify<T: Summary>(item1: &T, item2: &&T){}
//
//***********impl trait***************
//***********There two parameters have different type************
//pub fn notify(item1: & impl Summary, item2: &impl Summary)
//
//trait as parameter as generic as return type
