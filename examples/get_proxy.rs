use proxify::*;

pub fn main() {
    let proxy = get_proxy();
    let proxy = futures::executor::block_on(proxy);

    println!("{:?}", proxy);
    println!("");

    let working =
        futures::executor::block_on(check_proxies(&proxy, std::time::Duration::from_secs(2)));
    println!("{:?}", working);
}
