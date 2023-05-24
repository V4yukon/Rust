mod trait_interface;
use trait_interface:: {Summary, Tweet, NewsArticle};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you already know, people"
            ),
            reply: false,
            retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("China will get the championship of World Cup"),
        location: String::from("Chian HZ"),
        author: String::from("fatalchaos"),
        content: String::from("good good good"),
    };

    println!("New article {}", article.summarize());
    
    println!("1 new tweet: {}",tweet.summarize());

}

