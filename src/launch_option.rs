use std::time::Duration;
use headless_chrome::{LaunchOptions, LaunchOptionsBuilder};
use std::path::PathBuf;

pub fn stealth_option() -> LaunchOptions<'static> {
    let mut options = LaunchOptionsBuilder::default();
    options
        .headless(true)
        .window_size(Some((1920, 1080)))
        .args(vec![
            std::ffi::OsStr::new("--disable-blink-features=AutomationControlled"),
            std::ffi::OsStr::new("--disable-automation"),
            std::ffi::OsStr::new("--no-first-run"),
            std::ffi::OsStr::new("--disable-web-security"),
            std::ffi::OsStr::new("--disable-dev-shm-usage"),
            std::ffi::OsStr::new("--disable-browser-side-navigation"),
            std::ffi::OsStr::new("--disable-features=site-per-process,TranslateUI,BlinkGenPropertyTrees"),
            std::ffi::OsStr::new("--disable-popup-blocking"),
            std::ffi::OsStr::new("--disable-infobars"),
            std::ffi::OsStr::new("--disable-notifications"),
            std::ffi::OsStr::new("--disable-geolocation"),
            std::ffi::OsStr::new("--enable-webgl"),
            std::ffi::OsStr::new("--hide-scrollbars"),
            std::ffi::OsStr::new("--mute-audio"),
            std::ffi::OsStr::new("--no-sandbox"),
            std::ffi::OsStr::new("--disable-gpu"),
            std::ffi::OsStr::new("--ignore-certificate-errors"),
            std::ffi::OsStr::new("--user-agent=Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/98.0.4758.102 Safari/537.36")
        ]);
    options.build().expect("Failed to build launch options")
}

pub fn default_option() -> LaunchOptions<'static> {
    LaunchOptionsBuilder::default()
        .headless(false)
        .sandbox(true)
        .devtools(false)
        .enable_gpu(false)
        .enable_logging(false)
        .window_size(Some((1920, 1080)))
        .idle_browser_timeout(Duration::from_secs(60))
        .build()
        .unwrap()
}

pub fn with_proxy_port<'a>(options: LaunchOptions<'a>, port: usize) -> LaunchOptions<'a> {
    let proxy_url = format!("http://localhost:{}", port);
    let mut builder = LaunchOptionsBuilder::default();
    
    // Copy over existing options
    builder.headless(options.headless);
    builder.sandbox(options.sandbox);
    builder.devtools(options.devtools);
    builder.enable_gpu(options.enable_gpu);
    builder.enable_logging(options.enable_logging);
    
    if let Some(size) = options.window_size {
        builder.window_size(Some(size));
    }
    
    builder.idle_browser_timeout(options.idle_browser_timeout);
    
    // Add the proxy server
    builder.proxy_server(Some(&*Box::leak(proxy_url.into_boxed_str())))
        .build()
        .unwrap()
}

pub fn with_user_data_dir<'a>(options: LaunchOptions<'a>, path: &PathBuf) -> LaunchOptions<'a> {
    let mut builder = LaunchOptionsBuilder::default();
    
    // Copy over existing options
    builder.headless(options.headless);
    builder.sandbox(options.sandbox);
    builder.devtools(options.devtools);
    builder.enable_gpu(options.enable_gpu);
    builder.enable_logging(options.enable_logging);
    
    if let Some(size) = options.window_size {
        builder.window_size(Some(size));
    }
    
    builder.idle_browser_timeout(options.idle_browser_timeout);
    
    // Add the user data directory
    builder.user_data_dir(Some(path.clone()))
        .build()
        .unwrap()
}
