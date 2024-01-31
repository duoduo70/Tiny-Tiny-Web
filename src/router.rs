/* Tiny Tiny Web
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */
use crate::{config::*, drop::http::*, drop::log::LogLevel::*, i18n::LOG, macros::*, utils::*};
use std::sync::{atomic::Ordering, Arc, RwLock};

#[allow(clippy::type_complexity)]
static mut FILE_CACHE: Option<Arc<RwLock<(String, Vec<u8>)>>> = None;

pub fn router<'a>(
    req: HttpRequest<std::net::TcpStream>,
    res: &'a mut HttpResponse,
    config: &'a RouterConfig,
) -> bool {
    let serve_args = &config.serve_files_info;
    if serve_args.contains_key(&req.get_url().to_owned()) {
        res.set_header(
            "Content-Type",
            config
                .serve_files_info
                .get(&req.get_url().to_owned())
                .unwrap()
                .content_type
                .clone(),
        );
        let str = unsafe {
            match &FILE_CACHE {
                Some(a) => {
                    let str = if *req.get_url() == FILE_CACHE.get().0 {
                        FILE_CACHE.get().1
                    } else {
                        let _stream = match std::fs::read(
                            "export".to_owned()
                                + &config
                                    .serve_files_info
                                    .get(&req.get_url().to_owned())
                                    .unwrap()
                                    .file_path,
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
                                .serve_files_info
                                .get(&req.get_url().to_owned())
                                .unwrap()
                                .file_path,
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
            if let Some(replaces) = &k.replace {
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

    if ENABLE_CODE_NOT_FOUND.load(Ordering::Relaxed) {
        if let Some(res404) = &config.response_404 {
            *res = res404.clone();
            res.set_version("HTTP/1.1");
            res.set_state("404 NOT FOUND");
            res.set_header(
                "Content-Length",
                res.get_content_ref().clone().unwrap().len().to_string(),
            );
            return true;
        }

        res.set_version("HTTP/1.1");
        res.set_state("404 NOT FOUND");
        true
    } else {
        false
    }
}
fn router_iftype_replace<'a>(
    req: HttpRequest<std::net::TcpStream>,
    res: &'a mut HttpResponse,
    config: &'a RouterConfig,
    replaces: &Vec<ReplaceData>,
    str: String,
) -> bool {
    res.set_version("HTTP/1.1");
    res.set_state("200 OK");
    res.set_header(
        "Content-Type",
        config
            .serve_files_info
            .get(&req.get_url().to_owned())
            .unwrap()
            .content_type
            .clone(),
    );
    let mut final_str = String::new();
    for e in replaces {
        final_str = str.replace("$_gcflag", &e.content);
    }
    res.set_header("Content-Length", final_str.len().to_string());
    res.set_content(final_str.into());
    true
}
