extern crate reqwest;
extern crate select;
extern crate mysql;

use select::document::Document;
use select::predicate::Name;

fn main() {
    hacker_news("http://rurust.github.io/rust_book_ru/src/match.html");
}

fn hacker_news(url: &str) {
    return match reqwest::get(url).map(|t| {let mut a = t; a.text()}) {
        Ok(n) => println!("{}", n),
        Err(e) => println!("{}", "bad request"),
    }
}

