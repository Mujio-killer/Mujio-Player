// 导入reqwest库
use reqwest;

// 定义一个函数来发送HTTP GET请求获取XML信息
#[tauri::command]
pub fn fetch_rss(url: &str) -> Result<String, String> {
    // 发送同步的HTTP GET请求
    let response = reqwest::blocking::get(url);

    // 检查请求是否成功
    match response {
        Ok(res) => {
            // 读取响应的XML信息
            let xml = res.text();
            match xml {
                Ok(data) => Ok(data),  // 成功获取XML信息，返回给Vue3
                Err(_) => Err("Failed to read XML data".to_string())  // 读取XML信息失败
            }
        },
        Err(_) => Err("Failed to fetch RSS data".to_string())  // 请求RSS数据失败
    }
}