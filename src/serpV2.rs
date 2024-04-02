extern crate reqwest;
extern crate scraper;

use reqwest::Client;
use scraper::{Html, Selector};
use std::env;

async fn serp(search_query:&str, num:u8) -> Vec<String>{
    let client = Client::new();
    const BASE_URL: &str = "https://www.google.com/search";

    let mut search_urls:Vec<String> = Vec::new();

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
                        search_urls.push(search_url.to_string());
                    }
                }
            }
        }
    }

    search_urls.dedup(); // removes consecutive duplicates, if any.

    return search_urls;
}   

#[tokio::main] // main function is not allowed to be async, to counter that we are using tokio
async fn main() {
    let mut search = String::new();
    let mut num_pages:u8 = 3;

    let args: Vec<String> = env::args().collect();

    for i in 1..args.len() {
        match args[i].as_str() {
            "-s" => {
                if i + 1 < args.len() {
                    search = args[i + 1].clone();
                } else {
                    eprintln!("Missing search query after -s flag");
                    return;
                }
            }
            "-n" => {
                if i + 1 < args.len() {
                    num_pages = args[i + 1].parse().unwrap_or(2);
                } else {
                    eprintln!("Missing number of pages after -n flag");
                    return;
                }
            }
            _ => (),
        }
    }
    
    let search_urls_vec: Vec<_> = serp(search, num_pages).await;
    println!("{:?}", search_urls_vec);
} 
