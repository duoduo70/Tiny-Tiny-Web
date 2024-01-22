/* Tiny Tiny Web
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */

use super::*;
use super::macros::*;

pub fn func_for_each_eval(
    args: &[Expression],
    env: &mut Environment,
) -> Result<Expression, GError> {
    args_len_min!("for-each-eval", args, 2);
    args_len_max!("for-each-eval", args, 2);
    let from = check_type_onlyone!("for-each-eval", &args[0], env, List)?;
    let to = check_type_onlyone!("for-each-eval", &args[1], env, String)?;

    for e in from {
        let e_str = e.to_string();
        let (parsed_exp, _) = parse(&tokenize(to.replace("$$", &e_str[1..e_str.len()-1])))?;
        eval(&parsed_exp, env)?;
    }
    Ok(Expression::Bool(true))
}

pub fn func_eval(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("eval", args, 1);
    args_len_max!("eval", args, 1);
    let meta = check_type_onlyone!("eval", &args[0], env, String)?;

    let (parsed_exp, _) = parse(&tokenize(meta))?;
    Ok(eval(&parsed_exp, env)?)
}

pub fn func_meta(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("meta", args, 1);
    args_len_max!("meta", args, 1);
    let code = eval(&args[0], env)?;

    Ok(Expression::String(code.to_string()))
}

pub fn func_eval_atom(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("eval-atom", args, 1);
    args_len_max!("eval-atom", args, 1);
    let meta = check_type_onlyone!("eval-atom", &args[0], env, String)?;

    eval(&parse_atom(&meta), env)
}