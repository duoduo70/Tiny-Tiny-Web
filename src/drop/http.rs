/* Tiny-Tiny-Web/Drop
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License Version 3
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */
use std::{collections::HashMap, time::SystemTimeError};
pub struct HttpRequestError;
pub struct HttpRequest<'a, T> {
    request_method: String,
    url: String,
    version: String,
    headers: HashMap<String, String>,
    content: Option<std::iter::Take<std::io::Lines<std::io::BufReader<&'a mut T>>>>,
}
impl<'a, T> HttpRequest<'a, T> {
    pub fn new() -> Self {
        HttpRequest {
            request_method: String::new(),
            url: String::new(),
            version: String::new(),
            headers: HashMap::new(),
            content: None,
        }
    }
    pub fn from(str: String) -> Result<Self, HttpRequestError> {
        let mut request = HttpRequest::new();

        let mut req_line_flag = false;
        for line in str.lines() {
            if !req_line_flag {
                let mut req_line = line.split(' ');
                match req_line.nth(0) {
                    Some(a) => request.request_method = a.to_string(),
                    _ => return Err(HttpRequestError),
                }
                match req_line.nth(0) {
                    Some(a) => request.url = a.to_string(),
                    _ => return Err(HttpRequestError),
                }
                match req_line.nth(0) {
                    Some(a) => request.version = a.to_string(),
                    _ => return Err(HttpRequestError),
                }
                req_line_flag = true;
                continue;
            }

            let (k, v) = match line.split_once(':') {
                Some((k, v)) => (k, v),
                _ => match line.split_once(": ") {
                    Some((k, v)) => (k, v),
                    _ => {
                        return Err(HttpRequestError);
                    }
                },
            };
            request.headers.insert(k.to_string(), v.to_string());
        }
        Ok(request)
    }
    pub fn get_header(&self, str: String) -> Option<&String> {
        self.headers.get(&str)
    }
    #[allow(dead_code)]
    pub fn get_request_method(&self) -> &String {
        &self.request_method
    }
    pub fn get_url(&self) -> &String {
        &self.url
    }
    #[allow(dead_code)]
    pub fn get_version(&self) -> &String {
        &self.version
    }

    pub fn set_content(
        &mut self,
        content: Option<std::iter::Take<std::io::Lines<std::io::BufReader<&'a mut T>>>>,
    ) {
        self.content = content;
    }
}

#[derive(Clone)]
pub struct HttpResponse {
    version: String,
    state: String,
    headers: HashMap<String, String>,
    content: Option<Vec<u8>>,
}
impl HttpResponse {
    pub fn new() -> Self {
        HttpResponse {
            version: String::new(),
            state: String::new(),
            headers: HashMap::new(),
            content: None,
        }
    }
    pub fn set_version(&mut self, str: &str) {
        self.version = str.to_string()
    }
    pub fn set_state(&mut self, str: &str) {
        self.state = str.to_string()
    }
    pub fn set_header(&mut self, k: &str, v: String) -> Option<String> {
        self.headers.insert(k.to_string(), v)
    }
    pub fn set_content(&mut self, str: Vec<u8>) {
        self.content = Some(str)
    }
    pub fn get_content_ref(&self) -> &Option<Vec<u8>> {
        &self.content
    }
    pub fn get_content_unref(&self) -> Option<Vec<u8>> {
        self.content.clone()
    }
    pub fn get_stream(&self) -> Vec<u8> {
        let mut res: Vec<u8> = format!("{} {}\r\n", self.version, self.state)
            .as_bytes()
            .to_vec();
        for k in self.headers.keys() {
            res.extend(
                format!("{}: {}\r\n", k, self.headers.get(k).unwrap())
                    .as_bytes()
                    .to_vec(),
            )
        }
        res.push(b'\r');
        res.push(b'\n');
        if let Some(a) = &self.content {
            res.extend(a)
        }
        res
    }
    pub fn set_default_headers(&mut self, server: &str) -> Result<(), SystemTimeError> {
        let time = super::time::Time::new();
        self.headers.insert(
            "Date".to_string(),
            format!(
                "{}, {} {} {} {:0>2}:{:0>2}:{:0>2} GMT",
                time.wday_name()?,
                time.day()?,
                time.month_name()?,
                time.year()?,
                time.hour()?,
                time.min()?,
                time.sec()?
            ),
        );
        self.headers
            .insert("Server".to_string(), server.to_string());
        Ok(())
    }
}
