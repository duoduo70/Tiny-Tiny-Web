/* Tiny Tiny Web
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */

/// 该 trait 让 result_shldfatal 函数可以同时作用于 Option<T>, Result<T, E> 和其它的类型上
pub trait ShouldResult<T> {
    fn result_shldfatal(self, ret_code: i32, func: impl FnOnce() + std::marker::Send) -> T;
}
impl<T> ShouldResult<T> for Option<T> {
    fn result_shldfatal(self, ret_code: i32, func: impl FnOnce() + std::marker::Send) -> T {
        match self {
            Some(a) => a,
            _ => {
                func();
                std::process::exit(ret_code);
            }
        }
    }
}
impl<T, E> ShouldResult<T> for Result<T, E> {
    fn result_shldfatal(self, ret_code: i32, func: impl FnOnce() + std::marker::Send) -> T {
        match self {
            Ok(a) => a,
            _ => {
                func();
                std::process::exit(ret_code);
            }
        }
    }
}

/// 快速处理一个和时间相关的错误
/// 用以配合 time.rs
/// 如果出现了时间相关的错误，不会直接发生 panic ，而是会调用传入的闭包，然后以 ret_code 为程序的退出码
pub fn result_timeerr<T>(
    v: Result<T, std::time::SystemTimeError>,
    ret_code: i32,
    func: impl FnOnce() + std::marker::Send,
) -> T {
    v.result_shldfatal(ret_code, func)
}
