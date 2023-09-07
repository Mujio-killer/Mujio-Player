// 网络编程操作
use std::io::prelude::*;

struct CurlRequests {} // curl库的请求封装 doc: https://crates.io/crates/curl

impl CurlRequests {
    fn new() -> CurlRequests {
        return CurlRequests {};
    }

    fn print_response(&self, requests: &mut curl::easy::Easy) {
        // 统一输出响应体到终端
        requests
            .write_function(|data| {
                println!("{:?}", String::from_utf8_lossy(data));
                // std::io::stdout().write_all(data).unwrap();
                Result::Ok(data.len())
            })
            .unwrap();
        requests.perform().unwrap();
    }
    fn post_do(&self, requests: &mut curl::easy::Easy, data: &[u8]) {
        // post put 等请求封装
        let mut data_ = data;
        requests.post_field_size(data_.len() as u64).unwrap();
        let mut transfer = requests.transfer();
        transfer.read_function(|buf| Ok(data_.read(buf).unwrap()));
        transfer.perform().unwrap();
    }
    pub fn test_url(&self) {
        // 最简单的get请求
        let mut requests = curl::easy::Easy::new();
        requests.url("https://www.rust-lang.org/").unwrap();
        self.print_response(&mut requests);
        println!("{}", requests.response_code().unwrap());
    }
    pub fn test_get(&self) {
        // get请求
        let mut requests = curl::easy::Easy::new();
        requests.url("https://www.rust-lang.org/").unwrap();
        self.print_response(&mut requests);
        requests.get(true).unwrap();
        println!("{}", requests.response_code().unwrap());
    }
    pub fn test_post(&self) {
        // 模拟post请求
        let mut requests = curl::easy::Easy::new();
        let mut data = "nihao".as_bytes();
        requests.url("http://www.example.com/upload").unwrap();
        requests.post(true).unwrap();
        let mut header = curl::easy::List::new();
        header
            .append("Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ==")
            .unwrap();
        requests.http_headers(header).unwrap();
        self.post_do(&mut requests, data);
        println!("{}", requests.response_code().unwrap());
    }
}
use std::collections::HashMap;
struct Requests {} // 使用reqwest库doc: https://docs.rs/reqwest/0.11.9/reqwest/index.html

impl Requests {
    async fn test_get(&self) {
        let client = reqwest::Client::new();
        let body = client.get("https://www.rust-lang.org").send().await;
        println!("body = {:?}", body);
    }
    async fn test_post(&self) {
        // 表单格式
        let params = [("foo", "bar"), ("baz", "quux")];
        let client = reqwest::Client::new();
        let res = client
            .post("http://httpbin.org/post")
            .form(&params)
            .send()
            .await;
        println!("form {:?}", res);
        // json格式
        let mut map = HashMap::new();
        map.insert("lang", "rust");
        map.insert("body", "json");
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("HOST", "example.com".parse().unwrap());

        let client = reqwest::Client::new();
        let res = client
            .post("http://httpbin.org/post") // 请求路由
            .json(&map) // 数据
            .headers(headers) // 请求头
            .send()
            .await;
        println!("json {:?}", res);
    }
}

pub async fn test_curl() {
    // let mut req = CurlRequests::new();
    // req.test_post();
    let mut req = Requests {};
    // let mut a = req.test_get().await;
    let mut a = req.test_post().await;
}

