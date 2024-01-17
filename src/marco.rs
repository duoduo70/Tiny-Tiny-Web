/* Tiny Tiny Web
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */
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

macro_rules! create_static_string_list {
    ($n:ident, $($e:literal),*)=>{
        pub const $n: [&str; [$($e,)*].len()] = [
            $($e,)*
        ];
    };
}
pub(crate) use create_static_string_list;
