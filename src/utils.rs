/* Tiny Tiny Web
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */
use crate::drop::log::LogLevel::*;
use crate::macros::*;

pub trait TimeErr<T> {
    fn result_timeerr_default(self) -> T;
}
impl<T> TimeErr<T> for Result<T, std::time::SystemTimeError> {
    fn result_timeerr_default(self) -> T {
        crate::drop::tool::result_timeerr(self, -1, || log!(Fatal, crate::i18n::LOG[29]))
    }
}

pub trait GlobalValue<T> {
    fn get(self) -> T;
}
impl<T: Clone> GlobalValue<T> for &Option<std::sync::Arc<std::sync::Mutex<T>>> {
    fn get(self) -> T {
        self.clone().unwrap().lock().unwrap().clone()
    }
}
impl<T: Clone> GlobalValue<T> for &Option<std::sync::Arc<std::sync::RwLock<T>>> {
    fn get(self) -> T {
        self.clone().unwrap().read().unwrap().clone()
    }
}