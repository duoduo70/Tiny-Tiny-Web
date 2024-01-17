/* Tiny Tiny Web
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */
pub trait ShouldResult<T> {
    fn result_shldfatal(self, ret_code: i32, func: impl FnOnce() -> () + std::marker::Send) -> T;
}
impl<T> ShouldResult<T> for Option<T> {
    fn result_shldfatal(self, ret_code: i32, func: impl FnOnce() -> () + std::marker::Send) -> T {
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
    fn result_shldfatal(self, ret_code: i32, func: impl FnOnce() -> () + std::marker::Send) -> T {
        match self {
            Ok(a) => a,
            _ => {
                func();
                std::process::exit(ret_code);
            }
        }
    }
}

pub fn result_timeerr<T>(v: Result<T, std::time::SystemTimeError>, ret_code: i32, func: impl FnOnce() -> () + std::marker::Send) -> T {
    v.result_shldfatal(ret_code, func)
}