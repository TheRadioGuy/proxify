use proxify::*;
use tokio::prelude::*;

#[tokio::main]
pub async fn main() {
    let proxy = get_proxy().await.unwrap();
    println!("{:?}", proxy);
    println!("");

    let working = check_proxies(&proxy, std::time::Duration::from_secs(2)).await;
    println!("{:?}", working);
}
