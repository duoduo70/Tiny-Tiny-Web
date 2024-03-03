/* Tiny-Tiny-Web/Drop
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License Version 3
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */
use super::time::get_formatted_time;

/// 打印一条日志，通常来说，需要针对项目进行二次封装
/// lv: 日志级别，有 Info, Warn, Error, Fatal, Debug 五种选择
/// str: 要打印的字符串
/// enable_debug: 是否要开启 debug 模式，在 debug 模式下会额外打印方法名，行列号
/// fn_line_col_lctime: enable_debug 如果开启，则前三个参数是 enable_debug 的参数三元组
/// 第四个参数在不开启 enable_debug 的情况下也要使用，它控制着是否使用本地时间而非 UTC 时间
///
/// 如果选择使用本地时间，需要注意：获得时差的函数是不安全的，由 C 函数封装而来
///
/// 注：如果没有利用宏进行二次封装，则很难正常使用 debug 模式
/// 只要使用函数封装而非宏封装，就可以使用非 debug 模式
///
/// 如果没有开启 enable_debug ，就无法打印 Debug 级别的日志
/// 如果没有开启 enable_debug ，则 Debug 级别以外的日志会缺少方法名，行列号
///
/// 关于封装，Example:
/// ```
/// /* Tiny Tiny Web
/// * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
/// *
/// * You should have received a copy of the GNU General Public License
/// * along with this program;
/// * if not, see <https://www.gnu.org/licenses/>.
/// */
/// macro_rules! lgconf {
///    () => {
///        (
///            &std::file!()[4..],
///            std::line!(),
///            std::column!(),
///            crate::config::USE_LOCALTIME.load(std::sync::atomic::Ordering::Relaxed),
///        )
///    };
/// }
/// pub(crate) use lgconf;
///
/// macro_rules! log {
///    ($lv:ident, $str:expr) => {
///        crate::drop::log::log(
///            $lv,
///            ($str).to_string(),
///            crate::config::ENABLE_DEBUG.load(std::sync::atomic::Ordering::Relaxed),
///            lgconf!(),
///        )
///    };
/// }
/// pub(crate) use log;
///
/// macro_rules! process_result {
/// ($result:ident, $type:ident, $message:expr) => {{
///     let res: $type;
///     match $result {
///         Ok(res_) => res = res_,
///         Err(_) => {
///             log!(Fatal, $message);
///             panic!();
///         }
///     }
///     res
/// }};
/// }
/// pub(crate) use process_result;
/// ```
/// [该示例存档](https://github.com/duoduo70/Tiny-Tiny-Web/blob/ba0923fbe5a4c996f213e328ce2e0719fb93aaf0/src/macros.rs)
/// 在这个示例中，如果希望使用该宏:
/// ```
/// import drop::log::LogLevel::*; // 这个引入是必须的
/// import macros::*; // 示例宏被存储的地方
///
/// log!(Info, "This is a info.");
/// ```
pub fn log(
    lv: LogLevel,
    str: String,
    enable_debug: bool,
    fn_line_col_lctime: (&str, u32, u32, bool),
) {
    if lv == LogLevel::Debug && !enable_debug {
        return;
    }
    let time = get_formatted_time(fn_line_col_lctime.3);

    if lv == LogLevel::Debug {
        match time {
            Ok(a) => put_text(format!(
                "[{a}] [{lv}] [{}] [line:{}, column:{}] {str}",
                fn_line_col_lctime.0, fn_line_col_lctime.1, fn_line_col_lctime.2
            )),
            Err(_) => put_text(format!(
                "[VOIDTIME] [{lv}] [{}] [line:{}, column:{}] {str}",
                fn_line_col_lctime.0, fn_line_col_lctime.1, fn_line_col_lctime.2
            )),
        }
    } else {
        match time {
            Ok(a) => put_text(format!("[{a}] [{lv}] {str}")),
            Err(_) => put_text(format!("[VOIDTIME] [{lv}] {str}")),
        }
    }
}

/// 对打印文本操作的封装，更改这个函数可以让本库支持像 log4j 的打印到日志文件的操作
fn put_text(str: String) {
    println!("{str}");
}

/// 自动注册一个枚举的反射
/// 这个宏以后会被基于 stringify! 的新宏替代
macro_rules! enum_autoreflex {
    ($(#[$meta:meta])*$vis:vis $n:ident, $($e:ident),*, $($f:literal),*) => {
        $(#[$meta])*
        $vis enum $n {
            $($e,)*
        }
        impl std::fmt::Display for $n {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", {
                    match self {
                        $($n::$e =>$f,)*
                    }
                })
            }
        }
    };
}

enum_autoreflex! {
    #[derive(PartialEq)] pub LogLevel,
         Info,   Warn,   Error,    Fatal,   Debug,
        "INFO", "WARN", "ERROR",  "FATAL", "DEBUG"
}
