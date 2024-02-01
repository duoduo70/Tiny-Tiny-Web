/* Tiny Tiny Web
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */

use std::{
    io::Read, net::{TcpListener, TcpStream}, sync::{atomic::Ordering, Mutex}
};

use crate::{
    config::{Config, RouterConfig, ENABLE_CODE_BAD_REQUEST, XRPS_COUNTER_CACHE_SIZE}, drop::{
        http::{HttpRequest, HttpResponse},
        log::LogLevel::*,
    }, https::tls::{get_server_record_tls1_2_bytes, parse_has_record, CipherSuite, CompressionMethod, HandshakeServerHello, Random, RecordMessage}, i18n::LOG, macros::*, utils::TimeErr
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

pub fn handle_connection(mut stream: std::net::TcpStream, config: &Mutex<RouterConfig>) {
    use std::io::*;

    #[cfg(feature = "nightly")]
    {
    let mut buf = [0; 5];
    let _ = stream.read(&mut buf);
    if buf[0] == 22 {
        //https
        let record = crate::https::tls::RecordMessage::new(buf.into());
        if let Ok(a) = record {
            result_https_request(&stream, config, a)
        }
    } else if buf == "GET ".as_bytes() {
        //http
        result_http_request(stream, config)
    }
}
    #[cfg(not(feature = "nightly"))]
    result_http_request(stream, config)
}

fn result_http_request(mut stream: std::net::TcpStream, config: &Mutex<RouterConfig>) {
    let buf_reader = std::io::BufReader::new(&mut stream);

    let mut lines = std::io::BufRead::lines(buf_reader);

    let req_str = get_request_str(&mut lines);

    if req_str.is_empty() {
        if ENABLE_CODE_BAD_REQUEST.load(Ordering::Relaxed) {
            let mut response = HttpResponse::new();
            response.set_state("400 BAD REQUEST");
            write_stream(stream, &mut response);
        }
        return;
    }

    let mut request = if let Ok(req) = get_request(req_str) {
        req
    } else {
        return;
    };

    if let Some(a) = request.get_header("Content-Length".to_owned()) {
        request.set_content(Some(lines.take({
            match a.parse() {
                Ok(a) => a,
                _ => 0,
            }
        })))
    }

    let response = &mut HttpResponse::new();
    response
        .set_default_headers("Tiny-Tiny-Web/2")
        .result_timeerr_default();
    if !crate::router::router(request, response, &config.lock().unwrap()) {
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
    #[cfg(not(feature = "no-glisp"))]
    if enable_pipe {
        if let Some(content) = response.get_content_unref() {
            if let Ok(a) = std::str::from_utf8(&content) {
                pipe(config, a, enable_debug, response)
            }
        }
    }

    write_stream(stream, response)
}

// TODO: add https support
#[allow(dead_code)]
fn result_https_request(mut stream: &std::net::TcpStream, _config: &Mutex<RouterConfig>, record: RecordMessage) {
    let extra_length = record.length;
    let mut buf = vec![];
    if stream.take(extra_length.into()).read_to_end(&mut buf).is_err() {return};
    match parse_has_record( record, buf) {
        Ok(message) => {
            println!("{:#?}", message); // debug
            match message.handshake_message.handshake_content {
                crate::https::tls::HandshakeContent::HelloRequest => todo!(),
                crate::https::tls::HandshakeContent::ClientHello(client_msg) => {
                    let serverhello = HandshakeServerHello {
                        version: crate::https::tls::TLSVersion::TLS1_2,
                        random: Random::new_32bit_random(crate::drop::random::get_random_256().result_timeerr_default()),
                        session_id: client_msg.session_id,
                        ciper_suite: CipherSuite::TLS_AES_128_GCM_SHA256,
                        compression_method: CompressionMethod::Null,
                        extenssions_length: 0,
                    }.bytes();
                    let mut retvec = get_server_record_tls1_2_bytes(serverhello.len().try_into().unwrap());
                    retvec.extend(serverhello);
                    if std::io::Write::write_all(&mut stream, &retvec).is_err() {
                        log!(Debug, LOG[6])
                    }
                },
                crate::https::tls::HandshakeContent::ServerHello(_) => todo!(),
                crate::https::tls::HandshakeContent::Certificate => todo!(),
                crate::https::tls::HandshakeContent::ServerKeyExchange => todo!(),
                crate::https::tls::HandshakeContent::CertificateRequest => todo!(),
                crate::https::tls::HandshakeContent::ServerDone => todo!(),
                crate::https::tls::HandshakeContent::CertificateVerify => todo!(),
                crate::https::tls::HandshakeContent::ClientKeyExchange => todo!(),
                crate::https::tls::HandshakeContent::Finished => todo!(),
            }
        },
        Err(e) => match e {
            crate::https::tls::TLSError::RecodeTypeError(_) => println!("1"),
            crate::https::tls::TLSError::RecodeVersionError(_, _) => println!("2"),
            crate::https::tls::TLSError::HandshakeContentTypeError(_) => println!("3"),
            crate::https::tls::TLSError::UndefinedCiperSuite => println!("4"),
            crate::https::tls::TLSError::BadRequest => println!("5"),
        },
    }
}

fn get_request<'a>(req_str: String) -> Result<HttpRequest<'a, TcpStream>, ()> {
    if crate::config::ENABLE_DEBUG.load(Ordering::Relaxed) {
        match HttpRequest::from(req_str.clone()) {
            Ok(req) => {
                log!(Debug, format!("{}{}\n", LOG[7], req_str));
                Ok(req)
            }
            _ => {
                log!(Debug, format!("{}{}\n", LOG[5], req_str));
                Err(())
            }
        }
    } else {
        match HttpRequest::from(req_str) {
            Ok(req) => Ok(req),
            _ => Err(()),
        }
    }
}

fn get_request_str(lines: &mut std::io::Lines<std::io::BufReader<&mut TcpStream>>) -> String {
    let mut str = String::new();
    #[allow(clippy::never_loop)]
    loop {
        let line = lines.next();
        match line {
            Some(Ok(a)) => {
                if a.is_empty() {
                    break;
                };
                str += &(a + "\r\n")
            }
            _ => break,
        }
        break;
    }
    #[cfg(feature = "nightly")]
    return "GET ".to_owned() + &str;
    #[cfg(not(feature = "nightly"))]
    return str
}

fn write_stream(mut stream: TcpStream, response: &mut HttpResponse) {
    if std::io::Write::write_all(&mut stream, &response.get_stream()).is_err() {
        log!(Debug, LOG[6])
    }
}

fn pipe(
    config: &Mutex<RouterConfig>,
    content: &str,
    enable_debug: bool,
    response: &mut HttpResponse,
) {
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
                    return;
                }
            }
            Ok(crate::glisp::core::Expression::Bool(res)) => {
                log!(Info, format!("[{}] {} {}", LOG[32], LOG[33], res))
            }
            Ok(a) => {
                log!(Error, format!("[{}] {} {}", LOG[32], LOG[35], a));
                if crate::config::ENABLE_RETURN_IF_PIPE_ERR.load(Ordering::Relaxed) {
                    return;
                }
            }
        }
    }
}
