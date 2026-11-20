const BASE_PATH: &str = match option_env!("SITE_BASE_PATH") {
    Some(path) => path,
    None => "",
};

pub fn url(path: &str) -> String {
    format!(
        "{}/{}",
        BASE_PATH.trim_end_matches('/'),
        path.trim_start_matches('/')
    )
}
