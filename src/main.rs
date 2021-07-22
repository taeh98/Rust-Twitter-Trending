use tokio;
mod get_tweets;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let tweets: Vec<String> = get_tweets::get_recent_tweets().await;
}