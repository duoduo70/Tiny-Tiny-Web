/* Tiny Tiny Web
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */
use crate::{config::*, drop::http::*, drop::log::LogLevel::*, i18n::LOG, macros::*};
use std::sync::{Arc, RwLock};

static mut FILE_CACHE: Option<Arc<RwLock<(String, Vec<u8>)>>> = None;

pub fn router<'a>(
    req: HttpRequest<std::net::TcpStream>,
    res: &'a mut HttpResponse,
    config: &'a Config,
) -> bool {
    let serve_args = &config.serve_files_custom;
    if serve_args.contains_key(&req.get_url().to_owned()) {
        res.set_header(
            "Content-Type",
            match &config
                .serve_files_custom
                .get(&req.get_url().to_owned())
                .unwrap()
                .1
            {
                Some(a) => a.content_type.clone(),
                _ => "text/html; charset=utf-8".to_owned(),
            },
        );
        let str = unsafe {
            match &FILE_CACHE {
                Some(a) => {
                    let str = if &req.get_url().to_owned()
                        == &FILE_CACHE.clone().unwrap().read().unwrap().0
                    {
                        FILE_CACHE.clone().unwrap().read().unwrap().1.clone()
                    } else {
                        let _stream = match std::fs::read(
                            "export".to_owned()
                                + &config
                                    .serve_files_custom
                                    .get(&req.get_url().to_owned())
                                    .unwrap()
                                    .0,
                        ) {
                            Ok(a) => a,
                            _ => return false,
                        };

                        let mut lock = a.write().unwrap();
                        *lock = ("export".to_owned(), _stream.clone());
                        _stream
                    };

                    str
                }
                None => {
                    let _stream = std::fs::read(
                        "export".to_owned()
                            + &config
                                .serve_files_custom
                                .get(&req.get_url().to_owned())
                                .unwrap()
                                .0,
                    )
                    .unwrap();
                    FILE_CACHE = Some(Arc::new(RwLock::new((
                        "export".to_owned(),
                        _stream.clone(),
                    ))));
                    _stream
                }
            }
        };

        if let Some(k) = serve_args.get(&req.get_url().to_owned()) {
            if let Some(extra_args) = &k.1 {
                if let Some(replaces) = &extra_args.replace {
                    return router_iftype_replace(
                        req,
                        res,
                        config,
                        replaces,
                        match std::str::from_utf8(&str) {
                            Ok(v) => v.to_owned(),
                            Err(_) => {
                                log!(Debug, LOG[31]);
                                return false;
                            }
                        },
                    );
                }
            }
        }

        res.set_version("HTTP/1.1");
        res.set_state("200 OK");
        res.set_header("Content-Length", str.len().to_string());
        res.set_content(str);
        log!(
            Debug,
            format!("{}{}", LOG[14], "export".to_owned() + req.get_url())
        );
        return true;
    };

    if let Some(res404) = &config.response_404 {
        *res = res404.clone();
        res.set_version("HTTP/1.1");
        res.set_state("404 NOT FOUND");
        res.set_header(
            "Content-Length",
            res.get_content().clone().unwrap().len().to_string(),
        );
        return true;
    }

    res.set_version("HTTP/1.1");
    res.set_state("404 NOT FOUND");
    
    true
}
fn router_iftype_replace<'a>(
    //TODO: optimize it
    _req: HttpRequest<std::net::TcpStream>,
    res: &'a mut HttpResponse,
    _config: &'a Config,
    replaces: &Vec<(String, (usize, usize))>,
    _str: String,
) -> bool {
    res.set_version("HTTP/1.1");
    res.set_state("200 OK");
    res.set_header("Content-Type", "text/html;charset=utf-8".to_owned());
    let mut final_str = String::new();
    for e in replaces {
        final_str = _str.replace("$_gcflag", &e.0);
    }
    res.set_header("Content-Length", final_str.len().to_string());
    res.set_content(final_str.into());
    true
}
