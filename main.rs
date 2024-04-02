extern crate reqwest;
extern crate scraper;

use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::HashSet;

#[tokio::main] // main function is not allower to be asyn, to counter that we are using this
async fn main() {
    let search:&str = "latest news, sentiments and technical analysis on oil inda limited";
    let num_pages:u8 = 5;
    
    let search_urls_vec: Vec<_> = serp(search, num_pages).await;
    println!("{:?}", search_urls_vec);
}

async fn serp(search_query:&str, num:u8) -> Vec<String>{
    let client = Client::new();
    const BASE_URL: &str = "https://www.google.com/search";

    let mut search_urls = HashSet::new();

    let blacklist = vec![
        "https://www.google.com/search",
        "https://maps.google.com",
        "https://support.google.com",
    ];

    for page in 0..num {
        let url = if page == 0 {
            format!("{}?q={}", BASE_URL, search_query.replace(" ", "+"))
        } else {
            format!(
                "{}?q={}&start={}",
                BASE_URL,
                search_query.replace(" ", "+"),
                page * 10
            )
        };

        let response = client.get(&url).send().await.unwrap();
        let body = response.text().await.unwrap();

        let document = Html::parse_document(&body);
        let selector = Selector::parse("a").unwrap();

        for element in document.select(&selector) {
            if let Some(link) = element.value().attr("href") {
                if link.starts_with("/url?q=http"){
                    let mut search_url = link.split('=').nth(1).unwrap_or_default();
                    search_url = search_url.split("&").nth(0).unwrap_or_default();

                    if !blacklist.iter().any(|blacklisted_url| search_url.contains(blacklisted_url)){
                        search_urls.insert(search_url.to_string());
                    }
                }
            }
        }
    }

    let mut search_urls_vec: Vec<_> = search_urls.into_iter().collect();
    search_urls_vec.dedup(); // removes consecutive duplicates, if any.

    return search_urls_vec;
}   