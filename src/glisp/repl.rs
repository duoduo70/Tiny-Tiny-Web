/* Tiny Tiny Web
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */

use std::io::Write;

use super::core::*;
pub fn run_repl() {
    let env = &mut default_env();
    loop {
        print!("glisp > ");
        let _ = std::io::stdout().flush();
        let expr = slurp_expr();
        match parse_eval(expr, env) {
            Ok(res) => println!("; => {}", res),
            Err(e) => match e {
                GError::Reason(msg) => println!("; => {}", msg),
            },
        }
    }
}
