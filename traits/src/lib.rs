pub trait Summary {
    fn sumarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("Read more from {}...", self.sumarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn sumarize_author(&self) -> String {
        todo!()
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn sumarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
