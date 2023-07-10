use std::{
    fmt::{Debug, Display},
    iter::Sum,
};

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_strict<T: Summary>(item1: &T, item2: &T) {
    println!("Item1: {}, Item2: {}", item1.summarize(), item2.summarize());
}

pub fn notify_display(item: &(impl Summary + Display)) {
    println!("Item1: {}, Item2: {}", item.summarize(), item);
}

pub fn notify_display_two<T: Summary + Display>(item: &T) {
    println!("Item1: {}, Item2: {}", item.summarize(), item);
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn return_summarizable() -> impl Summary {
    Tweet {
        username: "Gambit".to_string(),
        content: "My first tweet".to_string(),
        reply: false,
        retweet: false,
    }
}

fn main() {
    let tweet = Tweet {
        username: "Gambit".to_string(),
        content: "My first tweet".to_string(),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let news_article = NewsArticle {
        headline: "Headline".to_string(),
        location: "Location".to_string(),
        author: "Author".to_string(),
        content: "Content".to_string(),
    };

    println!("New Article: {}", news_article.summarize());

    notify(&tweet);
    notify(&news_article);

    notify_strict(&tweet, &tweet);
    // notify_strict(&tweet, &news_article); Both have to be same type
}

fn _some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {}

fn _some_function_easy<T, U>(t: &T, u: &U)
where
    T: Display + Clone,
    U: Clone + Debug,
{
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("x is greater than y!")
        } else {
            println!("y is greater than x!")
        }
    }
}

trait Random {}

impl<T: Display> Random for T {
    //
}
