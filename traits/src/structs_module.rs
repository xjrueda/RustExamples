use crate::traits_module;

pub struct Book {
    pub headline : String,
    pub author : String,
    pub description : String,
}

impl traits_module::Summary for Book {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.description)
    }
    fn summarize_author(&self) -> String {
        format!("Created by: {}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
}

impl traits_module::Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn summarize_author(&self) -> String {
        format!("Tweeted by: {}", self.username)
    }
}

pub struct Article {
    pub author : String,
    pub subject : String,
}

impl traits_module::Summary for Article{
    fn summarize_author(&self) -> String {
        format!("Written by: {}", self.author)
    }
    fn summarize(&self) -> String {
        format!("{}: {}", self.subject, self.summarize_author())
    }
}

pub struct Paper {
    pub student :String,
    pub subject: String,
}

impl traits_module::Summary for Paper{
    fn summarize_author(&self) -> String {
        format!("Researcher : {}", self.student)
    } 
}
