/* Tiny Tiny Web
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */
use std::process::exit;

use crate::drop::log::LogLevel::*;
use crate::glisp::repl::run_repl;
use crate::i18n::LOG;
use crate::macros::*;

pub fn try_start() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1) {
        Some(a) => parse(a.to_string(), args),
        _ => return,
    }
}

fn parse(fst: String, _args: Vec<String>) {
    match fst.as_str() {
        #[cfg(not(feature = "no-glisp"))]
        "repl" => {
            run_repl();
        },
        "--version" => {
            println!("ttweb {}\nCopyright (C) 2024 Plasma, GPL Open Source Software.\nSee https://github.com/duoduo70/Tiny-Tiny-Web/.", env!("CARGO_PKG_VERSION"));
            exit(0);
        },
        _ => {
            log!(Fatal, LOG[30]);
            exit(-1);
        }
    }
}
