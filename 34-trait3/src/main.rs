use trait34::larger_value;
use trait34::notify;
use trait34::notify5;
use trait34::NewsArticle;
use trait34::Summary;
use trait34::Tweet;
fn main() {
    println!("Hello, world!");
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
    };

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best
        hockey team in the NHL.",
        ),
    };
    let str_list = vec!["hello", "very good"];

    println!("1 new tweet: {}", tweet.summarize());
    println!("2 new article: {}", article.summarize());
    notify(tweet);
    let new_article = notify5(article);
    println!("{}", new_article.summarize());
    println!("larger value is {}", larger_value(&str_list));
}
