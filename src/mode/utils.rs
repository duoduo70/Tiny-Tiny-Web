/* Tiny Tiny Web
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */

use std::{
    io::Read,
    net::{TcpListener, TcpStream},
    sync::{atomic::Ordering, Mutex},
};

use crate::{
    config::{
        Config, RouterConfig, ENABLE_CODE_BAD_REQUEST, SSL_CERTIFICATE, SSL_PRAVITE_KEY,
        XRPS_COUNTER_CACHE_SIZE,
    },
    drop::{
        http::{HttpRequest, HttpResponse},
        log::LogLevel::*,
        random::get_random_256,
    },
    https::tls::*,
    i18n::LOG,
    macros::*,
    utils::{GlobalValue, TimeErr},
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

#[allow(unused_mut)]
pub fn handle_connection(mut stream: std::net::TcpStream, config: &Mutex<RouterConfig>) {
    #[cfg(feature = "nightly")]
    {
        let mut buf = [0; 5];
        let _ = stream.read(&mut buf);
        if buf[0] == 22 {
            if unsafe { SSL_CERTIFICATE.is_none() } {
                todo!(); // TODO: add log and return
            }
            //https
            let record = crate::https::tls::RecordMessage::new(buf.into());
            if let Ok(a) = record {
                result_https_request(&stream, config, a)
            }
        } else if buf == "GET /".as_bytes() {
            // 因为读取 buf 时对原 Stream 进行了一次裁剪，所以在 get_request_str 函数中要把 "GET /" 加回去
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
        if let Some(content) = response.content_unref() {
            if let Ok(a) = std::str::from_utf8(&content) {
                pipe(config, a, enable_debug, response)
            }
        }
    }

    write_stream(stream, response)
}

fn get_tls_keys() -> (Vec<u8>, Vec<u8>) {
    let public_key: *mut u8 = [0; 32].as_mut_ptr();
    let pravite_key: *mut u8 = [0; 32].as_mut_ptr();
    let random = get_random_256().result_timeerr_default();
    let mut random_vec = random.0.to_be_bytes().to_vec();
    random_vec.extend(random.1.to_be_bytes());
    unsafe {
        crate::https::c25519::compact_x25519_keygen(
            pravite_key,
            public_key,
            random_vec.as_mut_ptr(),
        )
    };
    (
        crate::https::c25519::key_to_vec(pravite_key, 32),
        crate::https::c25519::key_to_vec(public_key, 32),
    )
}

// TODO: add https support
#[allow(dead_code)]
fn result_https_request(
    mut stream: &std::net::TcpStream,
    _config: &Mutex<RouterConfig>,
    record: RecordMessage,
) {
    let extra_length = record.length;
    let mut buf = vec![];
    if stream
        .take(extra_length.into())
        .read_to_end(&mut buf)
        .is_err()
    {
        return;
    };
    match parse_has_record(record, buf) {
        Ok(message) => {
            match message.handshake_message.handshake_content {
                crate::https::tls::HandshakeContent::HelloRequest => todo!(),
                crate::https::tls::HandshakeContent::ClientHello(client_msg) => {
                    println!("{:#?}", client_msg);
                    let serverhello_random = Random::new_32bit_random(
                        crate::drop::random::get_random_256().result_timeerr_default(),
                    );
                    let serverhello = HandshakeMessage {
                        handshake_content: HandshakeContent::ServerHello(HandshakeServerHello {
                            version: crate::https::tls::TLSVersion::TLS1_2,
                            random: serverhello_random,
                            session_id: client_msg.session_id,
                            ciper_suite: CipherSuite::TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA256,
                            compression_method: CompressionMethod::Null,
                            extenssions_length: 0,
                        }),
                        length: 0,
                    }
                    .bytes_without_length();
                    let mut retvec =
                        get_server_record_tls1_2_bytes(serverhello.len().try_into().unwrap());
                    retvec.extend(serverhello);
                    let certificate = HandshakeMessage {
                        handshake_content: HandshakeContent::Certificate(
                            HandshakeCertificate::new_just_one_certificate(unsafe {
                                SSL_CERTIFICATE.get().to_vec()
                            }),
                        ),
                        length: 0,
                    }
                    .bytes_without_length();
                    retvec.extend(get_server_record_tls1_2_bytes(
                        certificate.len().try_into().unwrap(),
                    ));
                    retvec.extend(certificate);
                    println!("{:#?}", retvec.len());
                    // ED25519 Signature (unstable)
                    let mut signature = [0; 64];
                    let sha_sign = unsafe {
                        let mut sign_public_key = [0; 32];
                        let mut sign_pravite_key = client_msg.random.bytes().to_vec();
                        sign_pravite_key.extend(SSL_PRAVITE_KEY.get().to_vec());
                        crate::https::c25519::compact_ed25519_calc_public_key(
                            sign_public_key.as_mut_ptr(),
                            sign_pravite_key.as_mut_ptr(),
                        );
                        sign_pravite_key.extend(sign_public_key);
                        let mut sha_arg = client_msg.random.bytes().to_vec();
                        sha_arg.extend(serverhello_random.bytes());
                        sha_arg.extend([0x03, 0x00, 0x1d]);
                        sha_arg.extend(sign_public_key);
                        crate::https::c25519::compact_ed25519_sign(
                            signature.as_mut_ptr(),
                            sign_pravite_key.as_mut_ptr(),
                            sha_arg.as_mut_ptr(),
                            sha_arg.len() as u32,
                        );
                        signature.clone().to_vec()
                    };

                    let (_pravite_key, public_key) = get_tls_keys();

                    let serverkeyexchange = HandshakeMessage {
                        handshake_content: HandshakeContent::ServerKeyExchange(
                            HandshakeServerKeyExchange {
                                curve_name: crate::https::tls::CurveName::X25519,
                                public_key,
                                sha_sign,
                            },
                        ),
                        length: 0,
                    }
                    .bytes_without_length();
                    retvec.extend(get_server_record_tls1_2_bytes(
                        serverkeyexchange.len().try_into().unwrap(),
                    ));
                    retvec.extend(serverkeyexchange);

                    let serverdone = HandshakeMessage {
                        handshake_content: HandshakeContent::HelloDone,
                        length: 0,
                    }
                    .bytes_without_length();
                    retvec.extend(get_server_record_tls1_2_bytes(
                        serverdone.len().try_into().unwrap(),
                    ));
                    retvec.extend(serverdone);
                    if std::io::Write::write_all(&mut stream, &retvec).is_err() {
                        log!(Debug, LOG[6])
                    }
                }
                crate::https::tls::HandshakeContent::ServerHello(_) => println!("1"),
                crate::https::tls::HandshakeContent::Certificate(_) => println!("2"),
                crate::https::tls::HandshakeContent::ServerKeyExchange(_) => println!("3"),
                crate::https::tls::HandshakeContent::CertificateRequest => println!("4"),
                crate::https::tls::HandshakeContent::HelloDone => println!("5"),
                crate::https::tls::HandshakeContent::CertificateVerify => println!("6"),
                crate::https::tls::HandshakeContent::ClientKeyExchange => println!("7"),
                crate::https::tls::HandshakeContent::Finished => println!("8"),
            }
        }
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
        match HttpRequest::from_string(req_str.clone()) {
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
        match HttpRequest::from_string(req_str) {
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
    return "GET /".to_owned() + &str;
    #[cfg(not(feature = "nightly"))]
    return str;
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
        match crate::glisp::core::parse_eval(e.to_string(), env, None) {
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
