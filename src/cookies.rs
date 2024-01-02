use std::{
    sync::OnceLock,
    time::{SystemTime, UNIX_EPOCH},
};
use url::Url;

pub static COOKIES: OnceLock<Vec<Cookie>> = OnceLock::new();
#[derive(Debug)]
pub struct Cookie {
    pub domain: String,
    pub tailmatch: bool,
    pub path: String,
    pub secure: bool,
    pub expiration: u64,
    pub name: String,
    pub value: String,
}
pub struct ParseCookieError {}

impl Cookie {
    pub fn encoded(&self) -> String {
        let dummy_url = format!("https://example.com/?{}={}", self.name, self.value);
        Url::parse(&dummy_url).unwrap().query().unwrap().to_owned()
    }

    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now();
        let since_epoch = now.duration_since(UNIX_EPOCH).expect("Time anomaly");

        self.expiration < since_epoch.as_secs()
    }

    pub fn matches_url(&self, url: &str) -> bool {
        match Url::parse(url) {
            Ok(url) => {
                match url.scheme() {
                    "http" => {
                        if self.secure {
                            return false;
                        }
                    }
                    "https" => {}
                    _ => return false,
                }
                if let Some(domain) = url.domain() {
                    if !domain.eq_ignore_ascii_case(&self.domain) {
                        return false;
                    }
                }
                if !url.path().starts_with(&self.path) {
                    return false;
                }
            }
            Err(_) => return false,
        }

        true
    }
}

pub fn parse_cookies(file_contents: &str) -> Result<Vec<Cookie>, ParseCookieError> {
    let cookie_lines = file_contents
        .lines()
        .filter(|l| !l.starts_with('#') && !l.is_empty());

    let mut cookies = vec![];
    for line in cookie_lines {
        match line.split('\t').collect::<Vec<&str>>().get(0..7) {
            Some(cookie_parts) => {
                let domain = cookie_parts[0].to_string();
                let tailmatch = cookie_parts[1] == "TRUE";
                let path = cookie_parts[2].to_string();
                let secure = cookie_parts[3] == "TRUE";

                let expiration: u64 = cookie_parts[4].parse().expect("Failed to parse expiration");

                let name = cookie_parts[5].to_string();
                let mut value = cookie_parts[6].to_string();
                // drop quotes
                value.remove(0);
                value.remove(value.len() - 1);

                cookies.push(Cookie {
                    domain,
                    tailmatch,
                    path,
                    secure,
                    expiration,
                    name,
                    value,
                });
            }
            None => return Err(ParseCookieError {}),
        }
    }
    Ok(cookies)
}
