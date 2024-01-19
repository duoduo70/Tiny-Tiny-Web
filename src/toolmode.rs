/* Tiny Tiny Web
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */
use std::process::exit;

#[cfg(not(feature = "stable"))]
use crate::glisp::compile::run_repl;
use crate::marco::*;
use crate::i18n::LOG;
use crate::drop::log::LogLevel::*;

pub fn tool_mode_try_start() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1) {
        Some(a) => parse(a.to_string(), args),
        _ => return
    }
}

fn parse(fst: String, _args: Vec<String>) {
    match fst.as_str() {
        #[cfg(not(feature = "stable"))]
        "repl" => {
            run_repl();
        },
        _ => {log!(Fatal, LOG[30]); exit(-1);}
    }
}