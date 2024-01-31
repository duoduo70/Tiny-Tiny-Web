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
    if let Some(a) = args.get(1) {
        parse(a.to_string(), args)
    }
}

fn parse(fst: String, _args: Vec<String>) {
    match fst.as_str() {
        #[cfg(not(feature = "no-glisp"))]
        "repl" => {
            run_repl();
        }
        "-v" | "--version" => {
            #[cfg(not(feature = "no-glisp"))]
            {
                #[cfg(not(feature = "nightly"))]
                println!("ttweb {}+glisp\nCopyright (C) 2024 Plasma, GPL Open Source Software.\nSee https://github.com/duoduo70/Tiny-Tiny-Web/.", env!("CARGO_PKG_VERSION"));
                #[cfg(feature = "nightly")]
                println!("ttweb {}+glisp+nightly\nCopyright (C) 2024 Plasma, GPL Open Source Software.\nSee https://github.com/duoduo70/Tiny-Tiny-Web/.", env!("CARGO_PKG_VERSION"));
            }
            #[cfg(feature = "no-glisp")]
            {
                #[cfg(not(feature = "nightly"))]
                println!("ttweb {}\nCopyright (C) 2024 Plasma, GPL Open Source Software.\nSee https://github.com/duoduo70/Tiny-Tiny-Web/.", env!("CARGO_PKG_VERSION"));
                #[cfg(feature = "nightly")]
                println!("ttweb {}+nightly\nCopyright (C) 2024 Plasma, GPL Open Source Software.\nSee https://github.com/duoduo70/Tiny-Tiny-Web/.", env!("CARGO_PKG_VERSION"));
            }
            exit(0);
        }
        "-h" | "--help" => {
            #[cfg(feature = "no-glisp")]
            println!("Warning: Cannot use glisp and repl because it is no-glisp version.");
            println!(
                r"Usage:
    ttweb
    ttweb repl
    ttweb -h | --help
    ttweb -v | --version"
            );
            println!(
                r"Options:
    repl            Start Ghost Lisp REPL
    -h --help       Show this screen.
    -v --version    Show version

    If you found a bug, see https://github.com/duoduo70/Tiny-Tiny-Web/ to report.
    If you don't know how to use this program, see:
        https://duoduo70.github.io/Tiny-Tiny-Web/,
        https://github.com/duoduo70/Tiny-Tiny-Web/blob/master/docs/index.md,
        Or <source-dir>/docs/ (According to GPLv3 license, you should be able to find it.)
            "
            );
            exit(0);
        }
        _ => {
            log!(Fatal, LOG[30]);
            exit(-1);
        }
    }
}
