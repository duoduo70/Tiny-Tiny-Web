/* Tiny-Tiny-Web/Drop
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License Version 3
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */
use std::{collections::HashMap, time::SystemTimeError};

/// 这个错误运用于一切可能的错误情况
/// 并不需要定义成枚举，因为该错误表示的意思是可以确定的
pub struct HttpRequestError;

/// 可以解析任意标准的 HTTP 请求字符串
///
/// request_method: 请求方法, 可能性有 `GET`, `HEAD`, `POST`, 'PUT' 等
/// url: 请求希望获取的页面的链接
/// version: HTTP 协议的版本，例如 `1.1`
/// headers: 该哈希表的键表示请求头的键，值表示请求头的值
/// content: 可选的，请求的主体部分，以缓冲区的方式储存
///
/// content 以缓冲区的方式储存的目的是避免过大的主体造成的一次性内存读取从而拖慢效率
///
/// 一个请求头的例子: `Content-Length: 32`，`Content-Length` 是键，`32` 是值
///
/// 参见[此文档](https://www.rfc-editor.org/rfc/rfc2616)
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
    /// 从一个字符串解析到 HttpRequest
    /// 传入的字符串不能包含 content ，而只可以包含请求行的请求头行
    /// 目前，需要使用 set_content 函数来设置请求主体
    ///
    /// TODO：这个算法应该能支持 content
    pub fn from_string(str: String) -> Result<Self, HttpRequestError> {
        if str.is_empty() {
            return Err(HttpRequestError);
        }

        let mut request = HttpRequest::new();
        let mut lines = str.lines();

        // 请求行
        let mut req_line = lines.next().unwrap().split(' ');
        match req_line.next() {
            Some(a) => request.request_method = a.to_string(),
            _ => return Err(HttpRequestError),
        }
        match req_line.next() {
            Some(a) => request.url = a.to_string(),
            _ => return Err(HttpRequestError),
        }
        match req_line.next() {
            Some(a) => request.version = a.to_string(),
            _ => return Err(HttpRequestError),
        }

        // 其它头部行
        for line in lines {
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
    pub fn request_method(&self) -> &String {
        &self.request_method
    }
    pub fn url(&self) -> &String {
        &self.url
    }
    #[allow(dead_code)]
    pub fn version(&self) -> &String {
        &self.version
    }

    pub fn set_content(
        &mut self,
        content: Option<std::iter::Take<std::io::Lines<std::io::BufReader<&'a mut T>>>>,
    ) {
        self.content = content;
    }
}

/// 可以构造一个标准的 HTTP 响应字符串
///
/// version: HTTP 相应的版本, 例如 `1.1`  
/// state: HTTP 相应的状态, 例如 `400 BAD REQUEST`  
/// header: 该哈希表的键表示相应头的键，值表示相应头的值  
/// content: 可选的，相应主体部分，以 `Vec<u8>` 的方式储存
///
/// 和 HttpRequest 不同, HttpResponse 用 `Vec<u8>` 的方式储存的原因是：  
/// 1. 需要经常改变 content 的值以计算出最终的 content ，直接用 `Vec<u8>` 来储存可以避免转换和内存拷贝  
/// 2. 对于大的相应主体，可以将其分成多个响应
///
/// 参见[此文档](https://www.rfc-editor.org/rfc/rfc2616)
#[derive(Clone, Default)]
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
    /// 根据不同需要，创建了 content_ref 和 content_unref 两个函数
    pub fn content_ref(&self) -> &Option<Vec<u8>> {
        &self.content
    }
    /// 根据不同需要，创建了 content_ref 和 content_unref 两个函数
    pub fn content_unref(&self) -> Option<Vec<u8>> {
        self.content.clone()
    }
    /// 以 `Vec<u8>` 的形式返回响应流
    /// 不使用 string 的原因是可以直接兼容标准库相关函数
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
    /// 在初始化后，随时为相应追加默认的相应头
    /// TODO：设计名为 set_default_headers_unstable 的函数来更快的追加默认响应头
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
