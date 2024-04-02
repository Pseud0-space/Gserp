extern crate reqwest;
extern crate scraper;

use reqwest::Client;
use scraper::{Html, Selector};

#[tokio::main] // main function is not allowed to be async, to counter that we are using tokio
async fn main() {
    let search:&str = "<YOUR_SEARCH_HERE>";
    let num_pages:u8 = <NUMBER_OF_PAGES_TO_SEARCH>;
    
    let search_urls_vec: Vec<String> = serp(search, num_pages).await;

    println!("[Search Query] >> {}", search);
    println!("[Number of Pages] >> {}\n", num_pages);
    println!("URLs:\n\n{:?}", search_urls_vec);
}

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
