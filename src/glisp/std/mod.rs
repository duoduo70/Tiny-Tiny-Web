/* Tiny Tiny Web
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */

mod core;
mod str;
mod macros;
mod io;
mod eval;

use super::core::*;
use core::*;
use str::*;
use io::*;
use eval::*;

pub fn eval_built_in_form(
    exp: &Expression,
    other_args: &[Expression],
    env: &mut Environment,
) -> Option<Result<Expression, GError>> {
    match exp {
        Expression::Symbol(symbol) => match symbol.as_ref() {
            "if" => Some(func_if(other_args, env)),
            "set" => Some(func_set(other_args, env)),
            "lambda" => Some(func_lambda(other_args)),
            "quote" => Some(func_quote(other_args)),
            "atom" => Some(func_atom(other_args, env)),
            "eq" => Some(func_eq(other_args, env)),
            "car" => Some(func_car(other_args, env)),
            "cdr" => Some(func_cdr(other_args, env)),
            "cons" => Some(func_cons(other_args)),
            "cond" => Some(func_cond(other_args, env)),
            "length" => Some(func_length(other_args, env)),
            "str.=" => Some(func_str_eq(other_args, env)),
            "str.!=" => Some(func_str_ne(other_args, env)),
            "str.<" => Some(func_str_lt(other_args, env)),
            "str.<=" => Some(func_str_le(other_args, env)),
            "str.>" => Some(func_str_gt(other_args, env)),
            "str.>=" => Some(func_str_ge(other_args, env)),
            "last" => Some(func_last(other_args, env)),
            "chars" => Some(func_chars(other_args, env)),
            "find" => Some(func_find(other_args, env)),
            "contains" => Some(func_contains(other_args, env)),
            "insert" => Some(func_insert(other_args, env)),
            "begin" => Some(func_begin(other_args, env)),
            "is-empty" => Some(func_is_empty(other_args, env)),
            "remove" => Some(func_remove(other_args, env)),
            "reverse" => Some(func_reverse(other_args, env)),
            "rfind" => Some(func_rfind(other_args, env)),
            "slice" => Some(func_slice(other_args, env)),
            "log" => Some(func_console_log(other_args, env)),
            "loop" => Some(func_loop(other_args, env)),
            "read-file" => Some(func_read_file(other_args, env)),
            "write-file" => Some(func_write_file(other_args, env)),
            "do" => Some(func_do(other_args, env)),
            "meta" => Some(func_meta(other_args, env)),
            "eval-atom" => Some(func_eval_atom(other_args, env)),
            "str" => Some(func_str(other_args, env)),
            "str.+" => Some(func_str_plus(other_args, env)),
            "or" => Some(func_or(other_args, env)),
            "and" => Some(func_and(other_args, env)),
            "lines" => Some(func_lines(other_args, env)),
            "return" => Some(Ok(Expression::Symbol("return".to_owned()))),
            "continue" => Some(Ok(Expression::Symbol("continue".to_owned()))),
            "pass" => Some(Ok(Expression::Symbol("pass".to_owned()))),
            "read-dir" => Some(func_read_dir(other_args, env)),
            "for-each-eval" => Some(func_for_each_eval(other_args, env)),
            "eval" => Some(func_eval(other_args, env)),
            _ => None,
        },
        _ => None,
    }
}
