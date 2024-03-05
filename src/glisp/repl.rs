/* Tiny Tiny Web
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */

use std::io::Write;

use super::core::*;
pub fn run_repl(enable_debug: bool) {
    if enable_debug {
        use crate::config::GLISP_DEBUG;
        GLISP_DEBUG.store(true, std::sync::atomic::Ordering::Relaxed);
        enable_stack()
    }
    let env = &mut default_env();
    loop {
        if enable_debug {
            print!("glisp-debug > ");
        } else {
            print!("glisp > ");
        }
        let _ = std::io::stdout().flush();
        let expr = slurp_expr();
        match parse_eval(expr, env, None) {
            Ok(res) => println!("; => {}", res),
            Err(e) => match e {
                GError::Reason(msg) => println!("; => {}", msg),
            },
        }
    }
}
