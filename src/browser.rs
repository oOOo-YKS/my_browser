use headless_chrome::Browser;
use crate::launch_option::stealth_option;
use std::collections::HashMap;
use std::error::Error;
use headless_chrome::protocol::cdp::Network::SetExtraHTTPHeaders;
use headless_chrome::protocol::cdp::Network::Headers;
use headless_chrome::protocol::cdp::Emulation::SetGeolocationOverride;
use headless_chrome::protocol::cdp::Emulation::SetTimezoneOverride;

pub struct HeadlessBrowser {
    pub browser: Browser,
}

#[derive(Debug)]
pub enum BrowserError {
    LaunchError(String),
    FetchError(String),
    NavigationError(String),
    NetworkError(String),
}

impl From<Box<dyn Error>> for BrowserError {
    fn from(error: Box<dyn Error>) -> Self {
        BrowserError::LaunchError(error.to_string())
    }
}
impl HeadlessBrowser {
    pub fn new() -> Result<Self, BrowserError> {
        Browser::new(stealth_option())  // 使用隐身配置
            .map(|browser| {
                // 初始化浏览器全局设置
                let tab = browser.new_tab().unwrap();
                
                // 设置地理位置
                tab.call_method(SetGeolocationOverride {
                    latitude: Some(31.2304),
                    longitude: Some(121.4737),
                    accuracy: Some(1.0),
                }).unwrap();
                
                // 设置时区
                tab.call_method(SetTimezoneOverride {
                    timezone_id: "Asia/Shanghai".into(),
                }).unwrap();
                
                // 添加默认请求头
                let mut default_headers = HashMap::new();
                default_headers.insert("accept".to_string(), "*/*".to_string());
                default_headers.insert("accept-encoding".to_string(), "gzip, deflate, br, zstd".to_string());
                default_headers.insert("accept-language".to_string(), "zh-CN,zh;q=0.9,en;q=0.8,en-GB;q=0.7,en-US;q=0.6".to_string());
                default_headers.insert("connection".to_string(), "keep-alive".to_string());
                default_headers.insert("sec-ch-ua".to_string(), "\"Microsoft Edge\";v=\"135\", \"Not-A.Brand\";v=\"8\", \"Chromium\";v=\"135\"".to_string());
                default_headers.insert("sec-ch-ua-mobile".to_string(), "?0".to_string());
                default_headers.insert("sec-ch-ua-platform".to_string(), "\"Windows\"".to_string());
                default_headers.insert("sec-fetch-dest".to_string(), "empty".to_string());
                default_headers.insert("sec-fetch-mode".to_string(), "cors".to_string());
                default_headers.insert("sec-fetch-site".to_string(), "cross-site".to_string());
                default_headers.insert(
                    "user-agent".to_string(), 
                    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/135.0.0.0 Safari/537.36 Edg/135.0.0.0".to_string()
                );
                
                let headers_obj = Headers(Some(serde_json::to_value(default_headers).unwrap()));
                tab.call_method(SetExtraHTTPHeaders { headers: headers_obj }).unwrap();
                
                HeadlessBrowser { browser }
            })
            .map_err(|e| BrowserError::LaunchError(e.to_string()))
    }

    pub fn fetch_page_html(&self, url: &str, headers: Option<HashMap<String, String>>) -> Result<String, BrowserError> {
        let tab = self.browser.new_tab()
            .map_err(|e| BrowserError::LaunchError(e.to_string()))?;

        // Set custom headers if provided
        if let Some(headers) = headers {
            let headers_obj = Headers(Some(serde_json::to_value(headers).map_err(|e| BrowserError::NetworkError(e.to_string()))?));
            tab.call_method(SetExtraHTTPHeaders { headers: headers_obj })
                .map_err(|e| BrowserError::NetworkError(e.to_string()))?;
        }

        // Navigate and get content
        tab.navigate_to(url)
            .map_err(|e| BrowserError::NavigationError(e.to_string()))?;
        
        tab.wait_until_navigated()
            .map_err(|e| BrowserError::NavigationError(e.to_string()))?;
        
        tab.get_content()
            .map_err(|e| BrowserError::FetchError(e.to_string()))
    }
}

mod tests {

    #[test]
    fn test_browser() {
        let browser = super::HeadlessBrowser::new().unwrap();
        let url = "https://www.sufe.edu.cn";
        let headers = Some(std::collections::HashMap::new());
        
        let html = browser.fetch_page_html(url, headers).unwrap();
        assert!(html.len() > 1000, "Failed to fetch page HTML");
    }
}