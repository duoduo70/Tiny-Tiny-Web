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
    drop::log::LogLevel::*, i18n::LOG, macros::*, Config, HttpRequest, HttpResponse, ShouldResult,
    TimeErr, ENABLE_DEBUG, XRPS_COUNTER_CACHE_SIZE,
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
    let config: Config = match crate::read_config("main.gc".to_owned(), &mut Config::new()) {
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
            std::net::ToSocketAddrs::to_socket_addrs(&address)
                .result_shldfatal(-1, || log!(Fatal, format!("{}{}", LOG[28], address)))
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

    let mut request = {
        if ENABLE_DEBUG.load(Ordering::Relaxed) {
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

    let mut response = HttpResponse::new();
    response
        .set_default_headers("Tiny-Tiny-Web/2")
        .result_timeerr_default();
    if !crate::router::router(request, &mut response, &config.lock().unwrap()) {
        return;
    }
    let content_stream = response.get_stream();
    if ENABLE_DEBUG.load(Ordering::Relaxed) {
        match std::str::from_utf8(&content_stream) {
            Ok(v) => log!(Debug, format!("{}{}\n", LOG[8], v)),
            Err(_) => log!(Debug, format!("{}{:?}\n", LOG[8], content_stream)),
        }
    }
    match stream.write_all(&response.get_stream()) {
        Err(_) => log!(Debug, LOG[6]),
        _ => (),
    }
}
