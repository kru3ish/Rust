// added dependenices in the cargo.toml
use futures::StreamExt;
use reqwest;
use std::error::Error;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://speed.hetzner.de/100MB.bin"; // A sample file for testing
    let file_name = "downloaded_file.bin";

    println!("Starting download from: {}", url);

    // Start downloading the file asynchronously
    let result = download_file(url, file_name).await;

    match result {
        Ok(_) => println!("Download complete! File saved as {}", file_name),
        Err(e) => eprintln!("Download failed: {}", e),
    }

    Ok(())
}

async fn download_file(url: &str, file_name: &str) -> Result<(), Box<dyn Error>> {
    // Make the HTTP GET request
    let response = reqwest::get(url).await?;
    
    // Create a new file asynchronously
    let mut file = File::create(file_name).await?;

    // Get the body as a stream
    let mut stream = response.bytes_stream();

    // Process the file in chunks
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk).await?;
        println!("Downloaded {} bytes...", chunk.len());
    }

    // Ensure all data is flushed and written
    file.flush().await?;

    Ok(())
}
