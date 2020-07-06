// use tokio::prelude::*;
use futures::executor::ThreadPool;
use isahc::prelude::*;
use scraper::{element_ref::ElementRef, Html, Selector};
use std::io::{Read, Write};
use std::net::SocketAddrV4;
use futures::future::join_all;
use std::str::FromStr;
use std::sync::{Arc, RwLock};
use std::time::Duration;

pub async fn get_proxy() -> Vec<SocketAddrV4> {
    let mut tasks = Vec::new();
    let proxies = Arc::new(RwLock::new(Vec::new()));
    
    for i in 1..=5 {
        let cloned = proxies.clone();
        let task = async move {
            let client = HttpClient::builder()
            .default_header("Accept", r#"text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9"#)
            .default_header("Accept-Encoding", "gzip, deflate")
            .default_header("Accept-Language", "ru-RU,ru;q=0.9,en-US;q=0.8,en;q=0.7")
            .default_header("Cache-Control", "max-age=0")
            .default_header("Connection", "keep-alive")
            .default_header("Host", "free-proxy.cz")
            .default_header("Cookie", "fp=f768e957238716fa2cc7232dfd308175")
            .default_header("Upgrade-Insecure-Requests", "1")
            .default_header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/83.0.4103.61 Safari/537.36")
            .build().unwrap();

            let mut response = client
                .get_async("http://free-proxy.cz/ru/proxylist/country/all/socks5/ping/all")
                .await.unwrap();
            let mut body = response.body_mut();
            let mut data = Vec::new();
            body.read_to_end(&mut data).unwrap();
            let text = String::from_utf8(data).unwrap();

            let selector = Selector::parse("table#proxy_list tbody tr").unwrap();
            let scraper = Html::parse_document(&text);
            let mut parsed_proxys: Vec<SocketAddrV4> = Vec::new();
            for element in scraper.select(&selector) {
                let inner_html = element.inner_html();
                let proxy_data = Html::parse_fragment(&inner_html);

                let decoded_ip = {
                    let encoded_ip_selector = Selector::parse("script").unwrap();
                    let encoded_ip_script = proxy_data
                .select(&encoded_ip_selector)
                .collect::<Vec<ElementRef>>()[0]
                .inner_html();

                    let encoded_ip = encoded_ip_script
                        .replace("document.write(Base64.decode(\"", "")
                        .replace("\"))", "");
                    String::from_utf8(base64::decode(&encoded_ip).unwrap()).unwrap()
                };

                let port = {
                    let port_selector = Selector::parse("span.fport").unwrap();
                    let port = proxy_data
                        .select(&port_selector)
                        .collect::<Vec<ElementRef>>();

                    match port.get(0) {
                        Some(port) => port.inner_html(),
                        None => "0".to_owned(),
                    }
                };

                if port != "0" {
                    cloned.write().unwrap().push(format!("{}:{}", decoded_ip, port).parse().unwrap());
                }
            }
        };

        tasks.push(task);
    }

    join_all(tasks).await;

    // println!("{}", text);

    Arc::try_unwrap(proxies).unwrap().into_inner().unwrap()
}

pub async fn check_proxies(proxies: &Vec<SocketAddrV4>, time: Duration) -> Vec<SocketAddrV4> {
    let working_proxys = Arc::new(RwLock::new(Vec::new()));
    let mut futures = Vec::new();
    for proxy in proxies {
        let formated = format!("socks5://{}", proxy);
        let proxy = proxy.clone();
        let worked_cloned = working_proxys.clone();

        let future = async move {
            let client = HttpClient::builder()
                .proxy(Some(formated.parse().unwrap()))
                .timeout(time)
                .build()
                .unwrap();
            let response = client.get_async("https://api.ipify.org?format=json").await;
            match response {
                Ok(_) => worked_cloned.write().unwrap().push(proxy),
                Err(e) => {}
            }
        };

        futures.push(future);
    }

    join_all(futures).await;
    Arc::try_unwrap(working_proxys)
        .unwrap()
        .into_inner()
        .unwrap()
}
