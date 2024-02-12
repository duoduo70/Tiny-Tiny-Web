/* Tiny Tiny Web
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */

use std::sync::{Arc, Mutex};
use std::{process::exit, sync::atomic::Ordering};

use super::utils::*;
use crate::config::Config;
use crate::drop::log::LogLevel::*;
use crate::drop::thread::ThreadPool;
use crate::i18n::LOG;
use crate::macros::*;
use crate::utils::GlobalValue;

pub fn start(config: Config) -> ! {

    log!(Info, LOG[15]);

    let listener = listener_init(config);

    let mut threadpool = ThreadPool::new();

    let threads_num = crate::config::THREADS_NUM.load(Ordering::Relaxed);

    for stream in listener.incoming() {
        match stream {
            Ok(req) => {
                threadpool.add(threads_num.try_into().unwrap(), || {
                    handle_connection(
                        req,
                        &Arc::new(Mutex::new(unsafe {
                            crate::config::GLOBAL_ROUTER_CONFIG.get()
                        })),
                    )
                });
            }
            Err(_) => {
                log!(Warn, LOG[4]);
                continue;
            }
        }
    }
    exit(0);
}
