use proxify::*;

pub fn main() {
    let proxy = get_proxy();
    let proxy = futures::executor::block_on(proxy).unwrap();

    println!("{:?}", proxy);
}
