/* Tiny Tiny Web
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */
use crate::drop::log::LogLevel::*;
use crate::{marco::*, LOG};

pub trait TimeErr<T> {
    fn result_timeerr_default(self) -> T;
}
impl<T> TimeErr<T> for Result<T, std::time::SystemTimeError> {
    fn result_timeerr_default(self) -> T {
        crate::drop::tool::result_timeerr(self, -1, || log!(Fatal, LOG[29]))
    }
}
