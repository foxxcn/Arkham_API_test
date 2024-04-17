use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use reqwest::Client;
use std::env;
use tokio::time::{sleep_until, Duration, Instant};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    dotenv().ok(); // 加载 .env 文件中的环境变量

    let api_key = env::var("API_KEY").expect("API_KEY not set"); // 从环境变量获取 API 密钥
    let url = "https://api.arkhamintelligence.com/transfers?base=ftx";

    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert("API-Key", HeaderValue::from_str(&api_key).unwrap());
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    // 初始化控制请求间隔的变量
    let mut next_request_time = Instant::now();
    let interval = Duration::from_millis(200); // 设定基本请求间隔为200毫秒

    for _ in 0..10 {
        // 举例，发送10个请求
        if Instant::now() < next_request_time {
            // 如果还没到下一个请求的时间，就等待
            sleep_until(next_request_time).await;
        }

        // 发送请求
        let response = client.get(url).headers(headers.clone()).send().await?;

        println!("请求状态: {}", response.status());

        // 计划下一个请求时间
        next_request_time = Instant::now() + interval;
    }

    Ok(())
}
