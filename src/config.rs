pub const OUTPUT_DIR: &str = "docs";
pub const STATIC_DIR: &str = "static";

pub const DEV_HOST: &str = "127.0.0.1";
pub const DEV_PORT: u16 = 1312;

pub const GITHUB_USERNAME: &str = "TynK-M";

const BASE_PATH: &str = match option_env!("SITE_BASE_PATH") {
    Some(path) => path,
    None => "",
};

pub fn url(path: &str) -> String {
    let base = BASE_PATH.trim_end_matches('/');
    let path = path.trim_start_matches('/');

    if path.is_empty() {
        format!("{base}/")
    } else {
        format!("{base}/{path}")
    }
}

fn site_url() -> String {
    match option_env!("SITE_URL") {
        Some(url) => url.to_string(),
        None => format!("http://{}:{}", DEV_HOST, DEV_PORT),
    }
}

pub fn absolute_url(path: &str) -> String {
    format!("{}{}", site_url().trim_end_matches('/'), url(path))
}
