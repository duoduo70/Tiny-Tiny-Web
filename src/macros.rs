/* Tiny Tiny Web
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */

/// 该宏只为 log! 宏需要的参数服务
macro_rules! lgconf {
    () => {
        (
            &std::file!()[4..],
            std::line!(),
            std::column!(),
            crate::config::USE_LOCALTIME.load(std::sync::atomic::Ordering::Relaxed),
        )
    };
}
pub(crate) use lgconf;

/// 该宏是对 log() 函数的封装
/// 使用例：
/// ```
/// import drop::log::LogLevel::*; // 这个引入是必须的
/// import macros::*; // 示例宏被存储的地方
/// 
/// log!(Info, "This is a info.");
/// ```
macro_rules! log {
    ($lv:ident, $str:expr) => {
        crate::drop::log::log(
            $lv,
            ($str).to_string(),
            crate::config::ENABLE_DEBUG.load(std::sync::atomic::Ordering::Relaxed),
            lgconf!(),
        )
    };
}
pub(crate) use log;

/// 用以处理一个 Result ，如果返回 Err(_) 会导致程序的无法再执行，可以使用本宏来创建一个附带 log 的 panic
macro_rules! process_result {
    ($result:ident, $type:ident, $message:expr) => {{
        let res: $type;
        match $result {
            Ok(res_) => res = res_,
            Err(_) => {
                log!(Fatal, $message);
                panic!();
            }
        }
        res
    }};
}
pub(crate) use process_result;

/// 目前为止，专门用以创建 i18n.rs 中的 LOG 数组，它有待修改，但并不紧急
/// 应该被修改成以有意义的字段表示本地化条目的方法，而非无意义的数字
/// 例如：
/// `LOG[0]` (即 "Tiny-Tiny-Web Started (Ver.")
/// 应该被修改成：
/// `LOG!(process_started)` 或类似的形式
macro_rules! create_static_string_list {
    ($n:ident, $($e:literal),*)=>{
        pub const $n: [&str; [$($e,)*].len()] = [
            $($e,)*
        ];
    };
}
pub(crate) use create_static_string_list;
