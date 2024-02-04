/* Tiny Tiny Web
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */

mod base;
mod vars;

use crate::config::base::*;
use crate::drop::http::HttpResponse;
use crate::drop::log::LogLevel::*;
use crate::i18n::LOG;
use crate::macros::*;
use core::sync::atomic::Ordering;
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicU32;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;

pub static USE_LOCALTIME: AtomicBool = AtomicBool::new(true);
pub static ENABLE_DEBUG: AtomicBool = AtomicBool::new(true);
pub static ENABLE_PIPE: AtomicBool = AtomicBool::new(false);
pub static THREADS_NUM: AtomicU32 = AtomicU32::new(2);
pub static XRPS_COUNTER_CACHE_SIZE: AtomicU32 = AtomicU32::new(8);
pub static BOX_NUM_PER_THREAD_MAG: AtomicU32 = AtomicU32::new(1000);
pub static BOX_NUM_PER_THREAD_INIT_MAG: AtomicU32 = AtomicU32::new(1000);
pub static XRPS_PREDICT_MAG: AtomicU32 = AtomicU32::new(1100);
pub static BOX_MODE: AtomicBool = AtomicBool::new(false);
pub static ENABLE_RETURN_IF_PIPE_ERR: AtomicBool = AtomicBool::new(true);
pub static ENABLE_CODE_BAD_REQUEST: AtomicBool = AtomicBool::new(false);
pub static ENABLE_CODE_NOT_FOUND: AtomicBool = AtomicBool::new(false);
pub static mut GLOBAL_ROUTER_CONFIG: Option<Arc<Mutex<RouterConfig>>> = None;
pub static mut SSL_CERTIFICATE: Option<Arc<RwLock<Vec<u8>>>> = None;
pub static mut SSL_PRAVITE_KEY: Option<Arc<RwLock<Vec<u8>>>> = None;
#[derive(Clone)]
pub struct ReplaceData {
    pub content: String,
    pub column: u32,
    pub line: u32,
}
#[derive(Clone)]
pub struct ServeFileData {
    pub file_path: String,
    pub content_type: String,
    pub replace: Option<Vec<ReplaceData>>,
}
impl ServeFileData {
    pub fn from(file_path: String, config: &Config) -> Self {
        ServeFileData {
            content_type: match file_path.rsplit('.').next() {
                Some(a) => Self::auto_content_type(a.to_owned(), config),
                _ => "application/octet-stream".to_owned(),
            },
            replace: None,
            file_path,
        }
    }
    pub fn from_with_content_type(file_path: String, content_type: String) -> Self {
        ServeFileData {
            content_type,
            replace: None,
            file_path,
        }
    }
    fn auto_content_type(ex_name: String, config: &Config) -> String {
        if let Some(mime_type) = config.mime_bind.get(&ex_name) {
            mime_type.to_string()
        } else {
            match ex_name.as_str() {
                "html" => "text/html",
                "css" => "text/css",
                "js" => "text/javascript",
                "gif" => "image/gif",
                "png" => "image/png",
                "jpg" => "image/jpeg",
                "jpeg" => "image/jpeg",
                "webp" => "image/webp",
                "svg" => "image/svg+xml",
                _ => "text/plain",
            }
            .to_owned()
        }
    }
}
#[derive(Clone)]
pub struct RouterConfig {
    pub serve_files_info: HashMap<String, ServeFileData>,
    pub response_404: Option<HttpResponse>,
    pub pipe: Vec<String>,
}
#[derive(Clone)]
pub struct Config {
    pub use_localtime: bool,
    pub enable_debug: bool,
    pub addr_bind: Vec<String>,
    pub router_config: RouterConfig,
    pub mime_bind: HashMap<String, String>,
    pub status_codes: Vec<u16>,
}
impl Config {
    pub fn new() -> Self {
        Config {
            use_localtime: true,
            enable_debug: false,
            addr_bind: vec![],
            router_config: RouterConfig {
                serve_files_info: HashMap::new(),
                response_404: None,
                pipe: vec![],
            },
            mime_bind: HashMap::new(),
            status_codes: vec![],
        }
    }
    pub fn sync_static_vars(&self) {
        USE_LOCALTIME.store(self.use_localtime, Ordering::Relaxed);
        ENABLE_DEBUG.store(self.enable_debug, Ordering::Relaxed);
        unsafe { GLOBAL_ROUTER_CONFIG = Some(Arc::new(Mutex::new(self.clone().router_config))) };
        if !self.router_config.pipe.is_empty() {
            ENABLE_PIPE.store(true, Ordering::Relaxed)
        }
        if self.status_codes.get(400).is_some() {
            ENABLE_CODE_BAD_REQUEST.store(true, Ordering::Relaxed)
        }
        if self.status_codes.get(404).is_some() {
            ENABLE_CODE_NOT_FOUND.store(true, Ordering::Relaxed)
        }
    }
    pub fn check(&self) {
        if self.router_config.serve_files_info.is_empty() {
            log!(Warn, LOG[13]);
        }
    }
}
pub fn read_config(filename: String, config: &mut Config) -> Result<&mut Config, ()> {
    let lines = if let Ok(lines) = read_lines("config/".to_owned() + &filename) {
        lines
    } else {
        log!(
            Error,
            format!("{}{}", LOG[9], "config/".to_owned() + &filename)
        );
        return Err(());
    };

    let mut line_number = 1;
    for line in lines {
        match line {
            Ok(str) => parse_line(
                str,
                config,
                &("config/".to_owned() + &filename),
                line_number,
            ),
            Err(_) => log!(
                Error,
                format!(
                    "{}{}{} {}{}",
                    LOG[10],
                    LOG[11],
                    "config/".to_owned() + &filename,
                    LOG[12],
                    line_number
                )
            ),
        }
        line_number += 1;
    }

    Ok(config)
}
