/* Tiny Tiny Web
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */

use super::macros::*;
use super::*;

pub fn func_for_each(
    args: &[Expression],
    env: &mut Environment,
    config: Config,
) -> Result<Expression, GError> {
    args_len_min!("for-each", args, 2);
    args_len_max!("for-each", args, 2);
    let from = check_type_onlyone!("for-each", &args[0], env, List, config.clone())?;

    if GLISP_DEBUG.load(std::sync::atomic::Ordering::Relaxed) && from.is_empty() {
        log!(Info, format!("[glisp-debugger] [lint] for-each got a empty list, so no any expression will construct"));
    }

    for e in from {
        let e_str = e.to_string();
        env.data.insert(
            "$$".to_owned(),
            crate::glisp::core::Expression::String(e_str),
        );
        eval(&args[1], env, config.clone())?;
    }
    Ok(Expression::Bool(true))
}

pub fn func_eval(
    args: &[Expression],
    env: &mut Environment,
    config: Config,
) -> Result<Expression, GError> {
    args_len_min!("eval", args, 1);
    args_len_max!("eval", args, 1);
    let meta = check_type_onlyone!("eval", &args[0], env, String, config.clone())?;

    let (parsed_exp, _) = parse(&tokenize(meta))?;
    eval(&parsed_exp, env, config)
}

pub fn func_meta(
    args: &[Expression],
    env: &mut Environment,
    config: Config,
) -> Result<Expression, GError> {
    args_len_min!("meta", args, 1);
    args_len_max!("meta", args, 1);
    let code = eval(&args[0], env, config)?;

    Ok(Expression::String(code.to_string()))
}

pub fn func_eval_atom(
    args: &[Expression],
    env: &mut Environment,
    config: Config,
) -> Result<Expression, GError> {
    args_len_min!("eval-atom", args, 1);
    args_len_max!("eval-atom", args, 1);
    let meta = check_type_onlyone!("eval-atom", &args[0], env, String, config.clone())?;

    eval(&parse_atom(&meta), env, config)
}