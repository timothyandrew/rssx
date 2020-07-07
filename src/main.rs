use std::io::{self, Read};
use std::error::Error;

fn get_urls() -> Result<Vec<String>, io::Error> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer.trim().split("\n").map(String::from).collect())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut feeds = rssx::fetch_all_feeds(get_urls()?).await?;

    // TODO: Don't clone
    feeds.sort_by_key(|f| f.title.clone());

    for feed in feeds.iter() {
        let md = rssx::serialize_feed(&feed);
        println!("{}", md);
    }

    Ok(())
}
