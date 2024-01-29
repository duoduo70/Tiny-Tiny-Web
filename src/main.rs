/* Tiny Tiny Web
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */
mod config;
mod drop;
mod i18n;
mod macros;
mod mode;
mod router;
mod utils;

mod glisp;

use drop::log::LogLevel::*;
use i18n::LOG;
use macros::*;

fn main() {
    mode::toolmode::try_start();

    log!(Info, format!("{}{}).", LOG[0], env!("CARGO_PKG_VERSION")));

    if config::BOX_MODE.load(std::sync::atomic::Ordering::Relaxed) {
        mode::boxmode::start();
    }

    mode::normalmode::start();
}
