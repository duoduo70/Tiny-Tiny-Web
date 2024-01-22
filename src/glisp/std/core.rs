/* Tiny Tiny Web
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */

use super::*;
use super::macros::*;

pub fn func_quote(args: &[Expression]) -> Result<Expression, GError> {
    let _fst = args
        .first()
        .ok_or(GError::Reason(format!("unexpected args form")))?;
    let mut retfst = vec![Expression::Symbol("quote".to_owned())];
    retfst.extend_from_slice(args);
    Ok(Expression::List(retfst))
}

pub fn func_atom(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    let fst = eval(
        args.first()
            .ok_or(GError::Reason(format!("unexpected args form")))?,
        env,
    )?;
    match fst {
        Expression::Symbol(_) => Ok(Expression::Bool(true)),
        Expression::Number(_) => Ok(Expression::Bool(true)),
        Expression::Func(_) => Ok(Expression::Bool(true)),
        Expression::Bool(_) => Ok(Expression::Bool(true)),
        Expression::String(_) => Ok(Expression::Bool(true)),
        Expression::List(a) => {
            let _fst = if let Some(e) = a.get(0) {
                e
            } else {
                return Ok(Expression::Bool(true));
            }
            .to_string();
            if _fst == *"quote" {
                Ok(Expression::Bool(false))
            } else {
                Ok(Expression::Bool(true))
            }
        }
        _ => Ok(Expression::Bool(false)),
    }
}

//TODO: Need to use args_len_xxx macro reconstruction
pub fn func_eq(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    let (fst, snd) = (
        args.first()
            .ok_or(GError::Reason(format!("eq can only have 2 args")))?,
        args.get(1)
            .ok_or(GError::Reason(format!("eq can only have 2 args")))?,
    );

    if eval(fst, env)?.to_string() == eval(snd, env)?.to_string() {
        Ok(Expression::Bool(true))
    } else {
        Ok(Expression::Bool(false))
    }
}

pub fn func_cons(args: &[Expression]) -> Result<Expression, GError> {
    args_len_min!("coud", args, 2);
    args_len_max!("coud", args, 2);

    let mut lst1 = match args[0].clone() {
        Expression::List(a) => a,
        _ => {
            return Err(GError::Reason(format!(
                "cons can only result a static list"
            )))
        }
    };

    if lst1.remove(0).to_string() != "quote" {
        return Err(GError::Reason(format!(
            "cons can only result a static list"
        )));
    }

    let mut lst2 = match args[1].clone() {
        Expression::List(a) => a,
        _ => {
            return Err(GError::Reason(format!(
                "cons can only result a static list"
            )))
        }
    };

    if lst2.remove(0).to_string() != "quote" {
        return Err(GError::Reason(format!(
            "cons can only result a static list"
        )));
    }

    lst1.extend(lst2);

    Ok(Expression::List(lst1))
}

pub fn func_cond(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("coud", args, 2);

    let mut i = 0;
    loop {
        if i >= args.len() {
            return Err(GError::Reason(format!("cond: Error2")));
        }

        let v = match eval(&args[i * 2].clone(), env) {
            Ok(a) => a,
            _ => return Err(GError::Reason(format!("cond: Error3"))),
        };
        match v {
            Expression::Bool(true) => return eval(&args[i * 2 + 1].clone(), env),
            _ => (),
        }
        i += 1;
    }
}

pub fn func_set(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("set", args, 2);
    args_len_max!("set", args, 2);
    let var_exp = &args[0];
    let val_res = &args[1];
    let evaled_val = eval(val_res, env)?;

    match var_exp {
        Expression::Symbol(ref var_name) => {
            env.data.insert(var_name.clone(), evaled_val);
            Ok(var_exp.clone())
        }
        _ => Err(GError::Reason(format!("unexpected var name"))),
    }
}

pub fn func_car(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("car", args, 1);
    args_len_max!("car", args, 1);
    let list = check_type_onlyone!("car", &args[0], env, List)?;
    if list.len() == 0 {
        Ok(Expression::List(vec![]))
    } else {
        Ok(list[0].clone())
    }
}

pub fn func_cdr(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("cdr", args, 1);
    args_len_max!("cdr", args, 1);
    let list = check_type_onlyone!("cdr", &args[0], env, List)?;
    if list.len() <= 1 {
        Ok(Expression::List(vec![]))
    } else {
        Ok(Expression::List(list[1..].to_vec()))
    }
}

pub fn func_loop(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("loop", args, 2);

    let mut i = 0;
    loop {
        if i >= args.len() {
            i = 0
        }
        let res = eval(&args[i], env)?;
        if res == Expression::Symbol("return".to_owned()) {
            break;
        }
        if res == Expression::Symbol("continue".to_owned()) {
            i = 0;
            continue;
        }

        i += 1;
    }

    Ok(Expression::Bool(true))
}

pub fn func_do(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("do", args, 1);

    let ret: &mut Expression = &mut Expression::List(vec![]);
    for e in args {
        *ret = eval(e, env)?;
    }
    Ok(ret.clone())
}

pub fn func_or(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("or", args, 2);
    args_len_max!("or", args, 2);
    let bool1 = check_type_onlyone!("or", &args[0], env, Bool)?;
    let bool2 = check_type_onlyone!("or", &args[1], env, Bool)?;

    Ok(Expression::Bool(bool1 || bool2))
}

pub fn func_and(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("and", args, 2);
    args_len_max!("and", args, 2);
    let bool1 = check_type_onlyone!("and", &args[0], env, Bool)?;
    if !bool1 {
        return Ok(Expression::Bool(false));
    }
    let bool2 = check_type_onlyone!("and", &args[1], env, Bool)?;

    Ok(Expression::Bool(bool2))
}