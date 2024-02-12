/* Tiny Tiny Web
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */

mod config;
mod core;
mod eval;
mod io;
mod macros;
mod str;

use super::core::*;
use config::*;
use core::*;
use eval::*;
use io::*;
use str::*;

pub fn eval_built_in_form(
    exp: &Expression,
    other_args: &[Expression],
    env: &mut Environment,
    config: Config,
) -> Option<Result<Expression, GError>> {
    match exp {
        Expression::Symbol(symbol) => match symbol.as_ref() {
            "if" => Some(func_if(other_args, env, config)),
            "set" => Some(func_set(other_args, env, config)),
            "lambda" => Some(func_lambda(other_args)),
            "quote" => Some(func_quote(other_args)),
            "atom" => Some(func_atom(other_args, env, config)),
            "eq" => Some(func_eq(other_args, env, config)),
            "car" => Some(func_car(other_args, env, config)),
            "cdr" => Some(func_cdr(other_args, env, config)),
            "cons" => Some(func_cons(other_args)),
            "cond" => Some(func_cond(other_args, env, config)),
            "length" => Some(func_length(other_args, env, config)),
            "str.=" => Some(func_str_eq(other_args, env, config)),
            "str.!=" => Some(func_str_ne(other_args, env, config)),
            "str.<" => Some(func_str_lt(other_args, env, config)),
            "str.<=" => Some(func_str_le(other_args, env, config)),
            "str.>" => Some(func_str_gt(other_args, env, config)),
            "str.>=" => Some(func_str_ge(other_args, env, config)),
            "last" => Some(func_last(other_args, env, config)),
            "chars" => Some(func_chars(other_args, env, config)),
            "find" => Some(func_find(other_args, env, config)),
            "contains" => Some(func_contains(other_args, env, config)),
            "insert" => Some(func_insert(other_args, env, config)),
            "begin" => Some(func_begin(other_args, env, config)),
            "is-empty" => Some(func_is_empty(other_args, env, config)),
            "remove" => Some(func_remove(other_args, env, config)),
            "reverse" => Some(func_reverse(other_args, env, config)),
            "rfind" => Some(func_rfind(other_args, env, config)),
            "slice" => Some(func_slice(other_args, env, config)),
            "log" => Some(func_console_log(other_args, env, config)),
            "loop" => Some(func_loop(other_args, env, config)),
            "read-file" => Some(func_read_file(other_args, env, config)),
            "write-file" => Some(func_write_file(other_args, env, config)),
            "do" => Some(func_do(other_args, env, config)),
            "meta" => Some(func_meta(other_args, env, config)),
            "eval-atom" => Some(func_eval_atom(other_args, env, config)),
            "str" => Some(func_str(other_args, env, config)),
            "str.+" => Some(func_str_plus(other_args, env, config)),
            "or" => Some(func_or(other_args, env, config)),
            "and" => Some(func_and(other_args, env, config)),
            "lines" => Some(func_lines(other_args, env, config)),
            "return" => Some(Ok(Expression::Symbol("return".to_owned()))),
            "continue" => Some(Ok(Expression::Symbol("continue".to_owned()))),
            "pass" => Some(Ok(Expression::Symbol("pass".to_owned()))),
            "read-dir" => Some(func_read_dir(other_args, env, config)),
            "for-each-eval" => Some(func_for_each_eval(other_args, env, config)),
            "eval" => Some(func_eval(other_args, env, config)),
            "run" => Some(func_run(other_args, env, config)),
            "serve" => Some(serve(other_args, env, config)),
            _ => None,
        },
        _ => None,
    }
}
