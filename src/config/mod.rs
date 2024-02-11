/* Tiny Tiny Web
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */

//! # 本模块的总则
//! Ghost Code 是一门致力于以最简单方式做配置的标记语言
//! 所以，任何高级语法都是不必要的
//! 所以，我们采用递归的方式来解析一个命令
//! 
//! 例如：
//! 我们有命令: `$ +addr 127.0.0.1` ，它会被 compile 函数在传入的字符串（配置文件的整个字符串）中被发现
//! 然后，该命令会被递交给 parse_line 函数，该函数用来解析一个个配置文件行
//! 因为它发现了该命令的第一项是 `$` ，该命令会被递交给相对应的 method_set 函数
//! 并将一些“相关数据”一并交给 method_set 函数
//! method_set 函数再将该命令附带“相关数据”传递给 pas_bool_option
//! MethodArgs 存储的就是这些“相关数据”，其中 config 和 line_splitted 是可变引用
//! 这样就可以直接在递归的内部修改 config ，而非使用很长的回调链造成性能问题和代码的可维护性问题
//! 
//! MethodArgs:
//! config: 需要被构造的配置文件
//! line_splitted: 分割后的参数字符串，例如在上面的那个例子里，是 `["+addr", "127.0.0.1"]`
//! file: 配置文件的文件名，主要用于报错信息的输出（如果最后发现一行配置包含一个错误，可以让报错输出更详细）
//! line_number: 同 file ，是该行配置之于配置文件的行号
//! 
//! 到此为止，我们阐明了整个模块的原理
//! 
//! ## 关于为本模块增加或删改功能
//! 原则上讲，应该尽可能遵循递归式处理的理念
//! 应该尽量使用 MethodArgs 而非额外定义
//! 
//! 因为 Ghost Code 不适用于复杂的功能
//! 如果要增加复杂的功能，应该为 Ghost Lisp 编程语言增加功能，然后增加 Ghost Code 语法作为接口
//! 
//! Ghost Code 和 Ghost Lisp 应该能被良好的对接
//! 事实上，完全可以将 Ghost Code 当作 Ghost Lisp 代码的“管理器”，让我们一眼就能明白该服务器有什么功能
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
pub static XRPS_COUNTER_CACHE_SIZE: AtomicU32 = AtomicU32::new(8);// 参见引用之处
pub static BOX_NUM_PER_THREAD_MAG: AtomicU32 = AtomicU32::new(1000);// 参见引用之处
pub static BOX_NUM_PER_THREAD_INIT_MAG: AtomicU32 = AtomicU32::new(1000);// 参见引用之处
pub static XRPS_PREDICT_MAG: AtomicU32 = AtomicU32::new(1100);// 参见引用之处
pub static BOX_MODE: AtomicBool = AtomicBool::new(false);// 参见引用之处
pub static ENABLE_RETURN_IF_PIPE_ERR: AtomicBool = AtomicBool::new(true);// 参见引用之处
pub static ENABLE_CODE_BAD_REQUEST: AtomicBool = AtomicBool::new(false);// 参见引用之处
pub static ENABLE_CODE_NOT_FOUND: AtomicBool = AtomicBool::new(false);// 参见引用之处
pub static mut GLOBAL_ROUTER_CONFIG: Option<Arc<Mutex<RouterConfig>>> = None;//每一个请求都会收到一个对其的引用
pub static mut SSL_CERTIFICATE: Option<Arc<RwLock<Vec<u8>>>> = None;//CA证书，这是 nightly 版本的一部分，仍在开发
pub static mut SSL_PRAVITE_KEY: Option<Arc<RwLock<Vec<u8>>>> = None;//公钥，这是nightly版本的一部分，以后会被移除

/// 该结构体用以存储一个 `$_grflags` 及其对应的元数据
/// 一个 OriginResponse 可能需要多个 ReplaceData ，因为这与 `$_grflags` 是一一对应的
/// 通常来说，OriginResponse = 文件
/// content: 要被替换的内容
/// column: `$_grflags` 在文件中的列号
/// line: `$_grflags` 在文件中的行号
#[derive(Clone)]
pub struct ReplaceData {
    pub content: String,
    pub column: u32,
    pub line: u32,
}

/// 该结构体用以存储一个被托管的文件对应的元数据
/// file_path: 被托管的文件的路径
/// content_type: 被托管的文件的 MIME 类型，例如 `application/octet-stream`
/// replace: 可选的，如果该文件里包含 `$_grflags` ，则存储它们及其对应的元数据
/// 
/// 关于 MIME 类型的标准名，参见：https://datatracker.ietf.org/doc/html/rfc6838
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

/// 这是 Router 的配置文件，每个请求都有一份引用或拷贝
/// 如果可能，应该尽量作为引用而非拷贝
#[derive(Clone)]
pub struct RouterConfig {
    pub serve_files_info: HashMap<String, ServeFileData>,
    pub response_404: Option<HttpResponse>,
    pub pipe: Vec<String>,
}

/// 这是总的配置文件，应该尽量避免拷贝，应该尽量保证唯一性，因为拷贝的代价很大
/// use_localtime: 是否使用本地时间而非 UTC 时间
/// enable_debug: 是否使用 debug 模式运行本程序，这主要跟日志的输出有关，debug 模式会极大的拖慢性能
/// addr_bind: 所有 IP 绑定的集合，例如 ["127.0.0.1:80", "127.0.0.1:22397", "[fe80::0]:80"]
/// mime_bind: 所有额外的 MIME 类型绑定的集合，键是文件后缀名，值的类型的标准名
/// status_codes: 启用的所有状态码，例如 [400, 404]
/// 
/// 关于 MIME 类型的标准名，参见：https://datatracker.ietf.org/doc/html/rfc6838
/// 关于所有的状态码，参见：https://datatracker.ietf.org/doc/html/rfc7231
/// 但是，本程序不支持所有状态码，关于已经支持了的状态码，参见本程序的文档或代码
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
    /// 对于部分选项，难以在它们的引用处给它们完整的 config
    /// 所以，一些选项不得已的要被推送到全局
    /// 这些被推送到全局的选项应该尽量少，应该尽量探索可能的不推送到全局的解决办法 
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
    /// 检查 Config 是否已经准备就绪
    pub fn check(&self) {
        if self.router_config.serve_files_info.is_empty() {
            log!(Warn, LOG[13]);
        }
    }
}
/// 读取一个配置文件簇的主配置文件
/// 例如，`main.gc`，我们将尽可能多个有关联（例如相互“引入”）的配置文件成为一个配置文件簇
/// 在主配置文件簇中，`main.gc`是主配置文件，因为它不被任何其它的配置文件引入
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
