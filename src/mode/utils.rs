/* Tiny Tiny Web
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */

use std::{
    net::TcpListener,
    sync::{atomic::Ordering, Mutex},
};

use crate::{
    config::{Config, XRPS_COUNTER_CACHE_SIZE},
    drop::{
        http::{HttpRequest, HttpResponse},
        log::LogLevel::*,
    },
    i18n::LOG,
    macros::*,
    utils::TimeErr,
};
use std::collections::VecDeque;

pub struct ReqCounter {
    req_num_per_sec: VecDeque<u32>,
    cache_size: u32,
}
impl ReqCounter {
    pub fn new() -> Self {
        let size = XRPS_COUNTER_CACHE_SIZE.load(Ordering::Relaxed);
        ReqCounter {
            req_num_per_sec: VecDeque::with_capacity(size.try_into().unwrap()),
            cache_size: size,
        }
    }
    pub fn get_xrps(&self) -> u32 {
        let mut num_full: u32 = 0;
        for e in self.req_num_per_sec.iter().collect::<Vec<_>>() {
            num_full += e;
        }
        num_full / self.cache_size
    }
    pub fn change(&mut self, new_num: u32) {
        self.req_num_per_sec.pop_front();
        self.req_num_per_sec.push_back(new_num);
    }
}

pub fn config_init() -> Config {
    let config: Config = match crate::config::read_config("main.gc".to_owned(), &mut Config::new())
    {
        Ok(config) => config.clone(),
        Err(_) => Config::new(),
    };
    config.check();
    config.sync_static_vars();

    config
}

pub fn listener_init(config: Config) -> TcpListener {
    let socket_addresses: Vec<std::net::SocketAddr> = config
        .addr_bind
        .iter()
        .map(|address| {
            crate::drop::tool::ShouldResult::result_shldfatal(
                std::net::ToSocketAddrs::to_socket_addrs(&address),
                -1,
                || log!(Fatal, format!("{}{}", LOG[28], address)),
            )
            .next()
            .unwrap()
        })
        .collect();

    let socket_addresses_array: &[std::net::SocketAddr] = socket_addresses.as_slice();
    let listener = TcpListener::bind(socket_addresses_array);

    process_result!(
        listener,
        TcpListener,
        format!("{}{:#?}", LOG[1], config.addr_bind)
    )
}

pub fn handle_connection(mut stream: std::net::TcpStream, config: &Mutex<Config>) {
    use std::io::*;

    let buf_reader = BufReader::new(&mut stream);
    let mut lines = std::io::BufRead::lines(buf_reader);

    let req_str = {
        let mut str = String::new();
        loop {
            let line = lines.next();
            match line {
                Some(Ok(a)) => {
                    if a == "" {
                        break;
                    };
                    str += &(a + "\r\n")
                }
                _ => break,
            }
            break;
        }
        str
    };

    if req_str.is_empty() {
        return;
    }

    let mut request = {
        if crate::config::ENABLE_DEBUG.load(Ordering::Relaxed) {
            match HttpRequest::from(req_str.clone()) {
                Ok(req) => {
                    log!(Debug, format!("{}{}\n", LOG[7], req_str));
                    req
                }
                _ => {
                    log!(Debug, format!("{}{}\n", LOG[5], req_str));
                    return;
                }
            }
        } else {
            match HttpRequest::from(req_str) {
                Ok(req) => req,
                _ => {
                    return;
                }
            }
        }
    };

    match request.get_header("Content-Length".to_owned()) {
        Some(a) => request.set_content(Some(lines.take({
            match a.parse() {
                Ok(a) => a,
                _ => 0,
            }
        }))),
        _ => (),
    };

    let mut response = &mut HttpResponse::new();
    response
        .set_default_headers("Tiny-Tiny-Web/2")
        .result_timeerr_default();
    if !crate::router::router(request, &mut response, &config.lock().unwrap()) {
        return;
    }

    let enable_pipe = crate::config::ENABLE_PIPE.load(Ordering::Relaxed);
    let enable_debug = crate::config::ENABLE_DEBUG.load(Ordering::Relaxed);
    if enable_debug {
        let content_stream = response.get_stream();
        match std::str::from_utf8(&content_stream) {
            Ok(v) => {
                if !enable_pipe {
                    log!(Debug, format!("{}{}\n", LOG[8], v))
                }
            }
            Err(_) => log!(Debug, format!("{}{:?}\n", LOG[8], content_stream)),
        }
    }
        #[cfg(not(feature = "stable"))]
        if enable_pipe {
            if let Some(content) = response.get_content() {
                match std::str::from_utf8(&content){
                    Ok(a) => pipe(config, a,enable_debug, &mut response),
                    Err(_) => {},
                }
            }
            
        }
    
    

    match stream.write_all(&response.get_stream()) {
        Err(_) => log!(Debug, LOG[6]),
        _ => (),
    }
}

fn pipe(config: &Mutex<Config>, content: &str, enable_debug: bool, response: &mut HttpResponse) {
    for e in &config.lock().unwrap().pipe {
        let env = &mut crate::glisp::core::default_env();
        env.data.insert(
            "CONTENT".to_owned(),
            crate::glisp::core::Expression::String(content.to_owned()),
        );
        match crate::glisp::core::parse_eval(e.to_string(), env) {
            Ok(crate::glisp::core::Expression::String(res)) => {
                if enable_debug {
                    log!(Debug, format!("{}{}\n", LOG[8], res));
                }
                response.set_content(res.clone().into());
                response.set_header("Content-Length", res.len().to_string());
            }
            Err(e) => {
                match e {
                    crate::glisp::core::GError::Reason(msg) => {
                        log!(Info, format!("[{}] {} {}", LOG[32], LOG[34], msg))
                    }
                }
                if crate::config::ENABLE_RETURN_IF_PIPE_ERR.load(Ordering::Relaxed) {
                    return
                }
            }
            Ok(crate::glisp::core::Expression::Bool(res)) => {
                log!(Info, format!("[{}] {} {}", LOG[32], LOG[33], res))
            }
            Ok(a) => {
                log!(Error, format!("[{}] {} {}", LOG[32], LOG[35], a));
                if crate::config::ENABLE_RETURN_IF_PIPE_ERR.load(Ordering::Relaxed) {
                    return
                }
            }
        }
    }
}