/* Tiny Tiny Web
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */
use crate::{config::*, drop::http::*, drop::log::LogLevel::*, i18n::LOG, macros::*};
use std::sync::atomic::Ordering;

/// 这是一个回调函数，返回值说明了本函数是否修改了 `res`
/// 如果请求不符合任何规则，则该函数返回 false
///
/// req: 传入的请求
/// res: 要被回调的相应
/// config: 一些给 Router 的配置文件
///
/// TODO: 该函数应该被 config 尽量的惰性构造来加快运行速度
/// TODO: 需要注释或重构
pub fn router<'a>(
    req: HttpRequest<std::net::TcpStream>,
    res: &'a mut HttpResponse,
    config: &'a RouterConfig,
) -> bool {
    let serve_args = &config.serve_files_info;
    if !serve_args.contains_key(&req.url().to_owned()) {
        return router_iftype_err(res, config);
    };

    res.set_header(
        "Content-Type",
        config
            .serve_files_info
            .get(&req.url().to_owned())
            .unwrap()
            .content_type
            .clone(),
    );
    let str = if let Some(content) = get_response_content(&req, config) {
        content
    } else {
        return false;
    };

    if let Some(k) = serve_args.get(&req.url().to_owned()) {
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
        format!("{}{}", LOG[14], "export".to_owned() + req.url())
    );

    true
}

fn get_response_content<'a>(
    req: &'a HttpRequest<std::net::TcpStream>,
    config: &'a RouterConfig,
) -> Option<Vec<u8>> {
    let _stream = std::fs::read(
        "export".to_owned()
            + &config
                .serve_files_info
                .get(&req.url().to_owned())
                .unwrap()
                .file_path,
    );
    Some(_stream.unwrap())
}

fn router_iftype_err<'a>(res: &'a mut HttpResponse, config: &'a RouterConfig) -> bool {
    if ENABLE_CODE_NOT_FOUND.load(Ordering::Relaxed) {
        if let Some(res404) = &config.response_404 {
            *res = res404.clone();
            res.set_version("HTTP/1.1");
            res.set_state("404 NOT FOUND");
            res.set_header(
                "Content-Length",
                res.content_ref().clone().unwrap().len().to_string(),
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
            .get(&req.url().to_owned())
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
