/* Tiny Tiny Web
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */
mod config;
mod counter;
mod drop;
mod i18n;
mod marco;
mod router;
mod tool;
mod toolmode;
mod boxmode;

#[cfg(not(feature = "stable"))]
mod glisp;

use config::*;
use counter::*;
use drop::http::*;
use drop::log::LogLevel::*;
use drop::thread::*;
use drop::tool::*;
use i18n::LOG;
use marco::*;
use tool::*;

use std::net::TcpListener;
use std::sync::atomic::Ordering;
use std::sync::Mutex;





fn main() {
    toolmode::tool_mode_try_start();

    log!(Info, format!("{}{}).", LOG[0], env!("CARGO_PKG_VERSION")));

    let config = config_init();

    log!(Info, LOG[15]);

    let listener = listener_init(config);

    let threadpool = ThreadPool::new();

    if config::BOX_MODE.load(Ordering::Relaxed) {
        crate::boxmode::boxmode(listener, threadpool);
    }

    let mut threadpool = ThreadPool::new();
    
    let threads_num = THREADS_NUM.load(Ordering::Relaxed);

    for stream in listener.incoming() {
        match stream {
            Ok(req) => {
                threadpool.add(threads_num.try_into().unwrap(), ||handle_connection(req, unsafe { &GLOBAL_CONFIG.clone().unwrap().clone() }));
            },
            Err(_) => todo!(),
        }
    }

}

fn config_init() -> Config {
    let config: Config = match read_config("main.gc".to_owned(), &mut Config::new()) {
        Ok(config) => config.clone(),
        Err(_) => Config::new(),
    };
    config.check();
    config.sync_static_vars();

    config
}

fn listener_init(config: Config) -> TcpListener {
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
