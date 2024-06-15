use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
}
// impl 一个trait 实现trait中的summarize具体方法
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
// item 类型使用impl Summary约束
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// item 类型使用trait bound 约束, +指定多个trait bound
pub fn notify2<T: Summary + Display>(item: T) {
    println!("Breaking news! {}", item.summarize());
}
// 多个参数
pub fn notify3<T: Summary + Display, U: Clone + Debug>(item: T, item2: U) {
    println!("Breaking news! {}", item.summarize());
}

// 使用where 约束T,U类型
pub fn notify4<T, U>(item1: T, item2: U) -> String
where
    T: Summary + Display,
    U: Clone + Debug,
{
    format!("breaknews, {}", item1.summarize())
}

pub fn notify5(item: impl Summary) -> impl Summary {
    NewsArticle {
        headline: String::from("headline"),
        location: String::from("location"),
        author: String::from("author"),
        content: String::from("content"),
    }
}
// 比较数组中的最大数
pub fn larger_value<T: PartialOrd + Clone>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
