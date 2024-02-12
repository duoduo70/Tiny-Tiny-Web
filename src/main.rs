/* Tiny Tiny Web
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */
mod config;
mod drop;
mod https;
mod i18n;
mod macros;
mod mode;
mod router;
mod utils;

mod glisp;

use config::Config;
use drop::log::LogLevel::*;
use i18n::LOG;
use macros::*;

fn main() {
    mode::toolmode::try_start();

    #[cfg(not(feature = "no-glisp"))]
    {
        #[cfg(not(feature = "nightly"))]
        log!(
            Info,
            format!("{}{}+glisp).", LOG[0], env!("CARGO_PKG_VERSION"))
        );
        #[cfg(feature = "nightly")]
        log!(
            Info,
            format!("{}{}+glisp+nightly).", LOG[0], env!("CARGO_PKG_VERSION"))
        );
    }
    #[cfg(feature = "no-glisp")]
    {
        #[cfg(not(feature = "nightly"))]
        log!(Info, format!("{}{}).", LOG[0], env!("CARGO_PKG_VERSION")));
        #[cfg(feature = "nightly")]
        log!(
            Info,
            format!("{}{}+nightly).", LOG[0], env!("CARGO_PKG_VERSION"))
        );
    }
    let config = config_init();

    if config::BOX_MODE.load(std::sync::atomic::Ordering::Relaxed) {
        mode::boxmode::start(config);
    }

    mode::normalmode::start(config);
}

pub fn config_init() -> Config {
    let config: Config = match crate::config::read_config("main.gc".to_owned(), &mut Config::new())
    {
        Ok(config) => config.clone(),
        Err(_) => Config::new(),
    };
    config.check();
    config.sync_static_vars();

    config
}
