use std::fmt::Display;

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub retweet: bool
}


impl Summary for Tweet {
//    fn summarize(&self) -> String {
//        format!("{}, by {}", self.username, self.content)
//    }
}

pub trait Summary {
    fn get_text(&self) -> String {
        return String::from("Text")
    }
    fn summarize(&self) -> String {
        format!("(Read more...) {}", self.get_text())
    }
}

fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// fn some_function <T, U>(t: &T, u: &U) -> i32
//     where T: Display + Clone,
//           U: Clone  {
//               49
// }

fn main() {
    let tweet = Tweet {
        username: String::from("username"),
        content: String::from("content"),
        retweet: true,
    };

    let article = NewsArticle {
        author: String::from("Test"),
        headline: String::from("headline"),
        content: String::from("content"),
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("Article summary: {}", article.summarize());
    notify(&article);
}
