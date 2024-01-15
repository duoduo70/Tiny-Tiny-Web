/* Tiny Tiny Web
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */
mod config;
mod drop;
mod marco;
mod router;
mod counter;

use config::*;
use drop::http::*;
use drop::log::LogLevel::*;
use drop::thread::*;
use marco::*;
use std::net::TcpListener;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};
use counter::*;
use std::collections::VecDeque;

use crate::drop::time::Time;

static mut THREADS_BOX: Option<Arc<Mutex<VecDeque<std::net::TcpStream>>>> = None;

fn main() {
    log!(Info, format!("{}{}).", LOG[0], env!("CARGO_PKG_VERSION")));

    let config: Config = match read_config() {
        Ok(config) => config,
        Err(_) => Config::new(),
    };
    config.check();
    config.sync_static_vars();

    let socket_addresses: Vec<std::net::SocketAddr> = config
        .addr_bind
        .iter()
        .map(|address| {
            std::net::ToSocketAddrs::to_socket_addrs(&address)
                .unwrap()
                .next()
                .unwrap()
        }) //TODO: fix it
        .collect();

    let socket_addresses_array: &[std::net::SocketAddr] = socket_addresses.as_slice();
    let listener = TcpListener::bind(socket_addresses_array);
    log!(Info, LOG[15]);

    let mut threadpool = ThreadPool::new();
    let res = process_result!(
        listener,
        TcpListener,
        format!("{}{:#?}", LOG[1], config.addr_bind)
    );
    match res.set_nonblocking(true) {
        Err(_) => log!(Warn, LOG[26]),
        _ => (),
    }

    let mut req_counter = ReqCounter::new();
    let mut old_stamp = Time::msec().unwrap(); //TODO: fix it. if cant get time, dont use box-mode to result requests
    let mut new_stamp: i16;
    let mut tmp_counter: u32 = 0;
    let mut box_num_per_thread: u32 = 0;
    unsafe { THREADS_BOX = Some(Arc::new(Mutex::new(VecDeque::new()))) };
    for stream in res.incoming() {
        match stream {
            Ok(stream) => {
                tmp_counter += 1;
                new_stamp = Time::msec().unwrap();
                let mut flag_new_box_num = false;
                if is_nst_gt_ost_helfsec(&old_stamp, &new_stamp) {
                    old_stamp = new_stamp;
                    req_counter.change(tmp_counter);
                    tmp_counter = 0;
                    box_num_per_thread = (req_counter.get_rps() as f32 * 1.1) as u32;
                    flag_new_box_num = true;
                }

                log!(Debug, format!("{}{:#?}\n", LOG[3], stream));
                    unsafe {
                        THREADS_BOX.clone().unwrap().clone().lock().unwrap().push_back(stream.try_clone().unwrap());
                        } // TODO: fix it
                    
                
                if flag_new_box_num || is_nst_gt_ost_timeout(&old_stamp, &new_stamp){
                    let func = move || {
                        let mut i = 0;
                        while i != box_num_per_thread {
                        handle_connection(unsafe { &THREADS_BOX.clone().unwrap() }, &Arc::clone(unsafe { &GLOBAL_CONFIG.clone().unwrap() }));
                        i+=1;
                        }
                    };
                    threadpool.add(6, func);
                    box_num_per_thread = 6 * 3;
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                continue;
            }
            _ => log!(Fatal, LOG[2]),
        }
    }
}

fn is_nst_gt_ost_timeout(old_stamp: &i16, new_stamp: &i16) -> bool {
    let differ = new_stamp - old_stamp;
    if differ > 50 {
        true
    } else if differ < 0 && 1000 + differ > 50 {
        true
    } else {
        false
    }
}

fn is_nst_gt_ost_helfsec(old_stamp: &i16, new_stamp: &i16) -> bool {
    let differ = new_stamp - old_stamp;
    if differ > 500 {
        true
    } else if differ < 0 && 1000 + differ > 500 {
        true
    } else {
        false
    }
}

fn handle_connection(streams: &Mutex<VecDeque<std::net::TcpStream>>, config: &Mutex<Config>) {
    use std::io::*;

    let mut stream = match streams.lock().unwrap().pop_front() {
        Some(a) => a,
        _ => return
    };

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
    response.set_default_headers("Tiny-Tiny-Web/2");
    if !crate::router::router(request, &mut response, &config.lock().unwrap()) {
        return;
    }
    if ENABLE_DEBUG.load(Ordering::Relaxed) {
        log!(Debug, format!("{}{}\n", LOG[8], response.get_str()));
    }
    match stream.write_all(response.get_str().as_bytes()) {
        Err(_) => log!(Debug, LOG[6]),
        _ => (),
    }
}
