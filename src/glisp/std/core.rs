/* Tiny Tiny Web
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */

use super::macros::*;
use super::*;

pub fn func_quote(args: &[Expression]) -> Result<Expression, GError> {
    let _fst = args
        .first()
        .ok_or(GError::Reason("unexpected args form".to_string()))?;
    let mut retfst = vec![Expression::Symbol("quote".to_owned())];
    retfst.extend_from_slice(args);
    Ok(Expression::List(retfst))
}

pub fn func_atom(
    args: &[Expression],
    env: &mut Environment,
    config: Config,
) -> Result<Expression, GError> {
    args_len_min!("atom", args, 1);
    args_len_max!("atom", args, 1);

    let fst = eval(&args[0], env, config)?;
    match fst {
        Expression::Symbol(_) => Ok(Expression::Bool(true)),
        Expression::Number(_) => Ok(Expression::Bool(true)),
        Expression::Func(_) => Ok(Expression::Bool(true)),
        Expression::Bool(_) => Ok(Expression::Bool(true)),
        Expression::String(_) => Ok(Expression::Bool(true)),
        Expression::List(a) => {
            let _fst = if let Some(e) = a.first() {
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

pub fn func_eq(
    args: &[Expression],
    env: &mut Environment,
    config: Config,
) -> Result<Expression, GError> {
    args_len_min!("eq", args, 2);
    args_len_max!("eq", args, 2);

    let fst = &args[0];
    let snd = &args[1];

    if eval(fst, env, config.clone())?.to_string() == eval(snd, env, config)?.to_string() {
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
            return Err(GError::Reason(
                "cons can only result a static list".to_string(),
            ))
        }
    };

    if lst1.remove(0).to_string() != "quote" {
        return Err(GError::Reason(
            "cons can only result a static list".to_string(),
        ));
    }

    let mut lst2 = match args[1].clone() {
        Expression::List(a) => a,
        _ => {
            return Err(GError::Reason(
                "cons can only result a static list".to_string(),
            ))
        }
    };

    if lst2.remove(0).to_string() != "quote" {
        return Err(GError::Reason(
            "cons can only result a static list".to_string(),
        ));
    }

    lst1.extend(lst2);

    Ok(Expression::List(lst1))
}

pub fn func_cond(
    args: &[Expression],
    env: &mut Environment,
    config: Config,
) -> Result<Expression, GError> {
    args_len_min!("coud", args, 2);

    let mut i = 0;
    loop {
        if i >= args.len() {
            return Err(GError::Reason("cond: Error2".to_string()));
        }

        let v = match eval(&args[i * 2].clone(), env, config.clone()) {
            Ok(a) => a,
            _ => return Err(GError::Reason("cond: Error3".to_string())),
        };
        if let Expression::Bool(true) = v {
            return eval(&args[i * 2 + 1].clone(), env, config);
        }
        i += 1;
    }
}

pub fn func_set(
    args: &[Expression],
    env: &mut Environment,
    config: Config,
) -> Result<Expression, GError> {
    args_len_min!("set", args, 2);
    args_len_max!("set", args, 2);
    let var_exp = &args[0];
    let val_res = &args[1];
    let evaled_val = eval(val_res, env, config)?;

    match var_exp {
        Expression::Symbol(ref var_name) => {
            env.data.insert(var_name.clone(), evaled_val);
            Ok(var_exp.clone())
        }
        _ => Err(GError::Reason("unexpected var name".to_string())),
    }
}

pub fn func_car(
    args: &[Expression],
    env: &mut Environment,
    config: Config,
) -> Result<Expression, GError> {
    args_len_min!("car", args, 1);
    args_len_max!("car", args, 1);
    let list = check_type_onlyone!("car", &args[0], env, List, config)?;
    if list.is_empty() {
        Ok(Expression::List(vec![]))
    } else {
        Ok(list[0].clone())
    }
}

pub fn func_cdr(
    args: &[Expression],
    env: &mut Environment,
    config: Config,
) -> Result<Expression, GError> {
    args_len_min!("cdr", args, 1);
    args_len_max!("cdr", args, 1);
    let list = check_type_onlyone!("cdr", &args[0], env, List, config)?;
    if list.len() <= 1 {
        Ok(Expression::List(vec![]))
    } else {
        Ok(Expression::List(list[1..].to_vec()))
    }
}

pub fn func_loop(
    args: &[Expression],
    env: &mut Environment,
    config: Config,
) -> Result<Expression, GError> {
    args_len_min!("loop", args, 1);

    let mut i = 0;
    loop {
        if i >= args.len() {
            i = 0
        }
        let res = eval(&args[i], env, config.clone())?;
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

pub fn func_do(
    args: &[Expression],
    env: &mut Environment,
    config: Config,
) -> Result<Expression, GError> {
    args_len_min!("do", args, 1);

    let ret: &mut Expression = &mut Expression::Bool(false);
    for e in args {
        *ret = eval(e, env, config.clone())?;
    }
    Ok(ret.clone())
}

pub fn func_or(
    args: &[Expression],
    env: &mut Environment,
    config: Config,
) -> Result<Expression, GError> {
    args_len_min!("or", args, 2);
    args_len_max!("or", args, 2);
    let bool1 = check_type_onlyone!("or", &args[0], env, Bool, config.clone())?;
    let bool2 = check_type_onlyone!("or", &args[1], env, Bool, config)?;

    Ok(Expression::Bool(bool1 || bool2))
}

pub fn func_and(
    args: &[Expression],
    env: &mut Environment,
    config: Config,
) -> Result<Expression, GError> {
    args_len_min!("and", args, 2);
    args_len_max!("and", args, 2);
    let bool1 = check_type_onlyone!("and", &args[0], env, Bool, config.clone())?;
    if !bool1 {
        return Ok(Expression::Bool(false));
    }
    let bool2 = check_type_onlyone!("and", &args[1], env, Bool, config)?;

    Ok(Expression::Bool(bool2))
}

pub fn func_repl(
    args: &[Expression],
    _env: &mut Environment,
    _config: Config,
) -> Result<Expression, GError> {
    args_len_max!("repl", args, 1);
    if args.is_empty() {
        crate::glisp::repl::run_repl(false);
        return Ok(Expression::Bool(true));
    }
    if let Expression::List(list) = &args[0] {
        if list.len() == 1 && list[0].to_string() == ":debug" {
            crate::glisp::repl::run_repl(true);
            return Ok(Expression::Bool(true));
        }
    }

    Ok(Expression::Bool(false))
}

pub fn func_fly(
    args: &[Expression],
    env: &mut Environment,
    config: Config,
) -> Result<Expression, GError> {
    args_len_min!("fly", args, 1);
    args_len_max!("fly", args, 2);
    let symbol = match &args[0] {
        Expression::Symbol(symbol) => symbol.to_string(),
        Expression::List(_) => return Err(GError::Reason("fly: WIP feature: the type of the first arg is a list".to_owned())),
        _ => return Err(GError::Reason("fly: the type of the first arg is not supported".to_owned()))
    };

    let mut times: u32 = 1;
    if args.len() == 2 {
        times = check_type_onlyone!("fly", &args[1], env, Number, config)? as u32;
    }
    let exp = env.data.remove(&symbol).ok_or(GError::Reason("fly: unknown symbol in the environment".to_owned()))?;

    let mut env = env.outer.ok_or(GError::Reason("fly: the environment is not so many layers".to_owned()))?;
    
    while times > 1 {
        env = env.outer.ok_or(GError::Reason("fly: the environment is not so many layers".to_owned()))?;
        times -= 1;
    }

    unsafe {
    let env = 
        env as *const Environment as *mut Environment; // 我知道我在干什么。这里使用它非常安全。
        (*env).data.insert(symbol, exp);
    }

    Ok(Expression::Bool(true))
}

pub fn func_drop(
    args: &[Expression],
    env: &mut Environment,
    _config: Config,
) -> Result<Expression, GError> {
    args_len_min!("drop", args, 1);
    args_len_max!("drop", args, 1);
    let symbol = match &args[0] {
        Expression::Symbol(symbol) => symbol.to_string(),
        Expression::List(_) => return Err(GError::Reason("drop: WIP feature: the type of the first arg is a list".to_owned())),
        _ => return Err(GError::Reason("drop: the type of the first arg is not supported".to_owned()))
    };

    env.data.remove(&symbol);

    Ok(Expression::Bool(true))
}

pub fn func_space(
    args: &[Expression],
    env: &mut Environment,
    config: Config,
) -> Result<Expression, GError> {
    args_len_min!("space", args, 1);

    let mut new_env = Environment {
        data: std::collections::HashMap::new(),
        outer: Some(env)
    };

    let ret: &mut Expression = &mut Expression::Bool(false);
    for e in args {
        *ret = eval(e, &mut new_env, config.clone())?
    }

    Ok(ret.clone())
}

pub fn func_snatch(
    args: &[Expression],
    env: &mut Environment,
    config: Config,
) -> Result<Expression, GError> {
    args_len_min!("snatch", args, 1);
    args_len_max!("snatch", args, 2);
    let symbol = match &args[0] {
        Expression::Symbol(symbol) => symbol.to_string(),
        Expression::List(_) => return Err(GError::Reason("snatch: WIP feature: the type of the first arg is a list".to_owned())),
        a => return Err(GError::Reason("snatch: the type of the first arg is not supported: ".to_owned() + &a.to_string()))
    };

    let mut times: u32 = 1;
    if args.len() == 2 {
        times = check_type_onlyone!("snatch", &args[1], env, Number, config)? as u32;
    }

    let mut outer_env = env.outer.ok_or(GError::Reason("snatch: the environment is not so many layers".to_owned()))?;
    
    while times > 1 {
        outer_env = outer_env.outer.ok_or(GError::Reason("snatch: the environment is not so many layers".to_owned()))?;
        times -= 1;
    }

    let exp = unsafe {
    let outer_env = 
        outer_env as *const Environment as *mut Environment; // 我知道我在干什么。这里使用它非常安全。
        (*outer_env).data.remove(&symbol)
    }.ok_or(GError::Reason("snatch: can not find the symbol: ".to_owned() + &symbol))?;
    env.data.insert(symbol, exp);

    Ok(Expression::Bool(true))
}