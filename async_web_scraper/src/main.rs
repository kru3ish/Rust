// added dependenicies to the cargo.toml file
use reqwest;
use scraper::{Html, Selector};
use std::error::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // URLs to scrape
    let urls = vec![
        "https://www.rust-lang.org/",
        "https://www.mozilla.org/",
        "https://www.github.com/",
    ];

    let mut handles = vec![];

    // Loop through the URLs
    for url in urls {
        let handle = tokio::spawn(async move {
            if let Err(e) = scrape_website(url).await {
                eprintln!("Error scraping {}: {}", url, e);
            }
        });
        handles.push(handle);
    }

    // Await all handles
    for handle in handles {
        handle.await?;
    }

    Ok(())
}

async fn scrape_website(url: &str) -> Result<(), Box<dyn Error>> {
    println!("Scraping: {}", url);

    // Send GET request to the URL
    let body = reqwest::get(url).await?.text().await?;

    // Parse the HTML
    let document = Html::parse_document(&body);
    let selector = Selector::parse("a").unwrap();

    // Extract and print the links
    for element in document.select(&selector) {
        if let Some(link) = element.value().attr("href") {
            println!("Found link: {}", link);
        }
    }

    Ok(())
}
