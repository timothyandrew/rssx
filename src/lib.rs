use atom_syndication::Feed;
use std::error::Error;
use futures::future::join_all;

pub fn serialize_feed(feed: &Feed) -> String {
    let mut buf = String::new();
    let authors = feed.authors.iter().map(|a| a.name.to_string()).collect::<Vec<_>>();

    buf.push_str(&format!("## [{}]({})\n", feed.title, feed.id));
    buf.push_str(&format!("\n*By {}; last updated: {}*\n\n", authors.join(","), feed.updated));

    for entry in feed.entries.iter() {
        buf.push_str(&format!("* [{}]({})\n", entry.title, entry.id));
    }

    buf
}

async fn fetch_feed(url: &str) -> Result<Feed, Box<dyn Error>> {
    let content = reqwest::get(url).await?.bytes().await?;
    let channel = Feed::read_from(&content[..])?;
    Ok(channel)
}

pub async fn fetch_all_feeds(urls: Vec<String>) -> Result<Vec<Feed>, Box<dyn Error>> {
    let futures = urls.iter().map(|url| {
        fetch_feed(url)
    });

    let feeds = join_all(futures).await;
    let feeds = feeds.into_iter().map(|feed| feed.unwrap());
    let feeds: Vec<Feed> = feeds.collect();

    Ok(feeds)
}