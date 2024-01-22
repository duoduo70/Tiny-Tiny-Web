/* Tiny Tiny Web
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */

use super::macros::*;
use super::*;
pub fn func_length(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("length", args, 1);
    args_len_max!("length", args, 1);
    let arg_res = check_type_onlyone!("length", &args[0], env, String);
    match arg_res {
        Ok(str) => Ok(Expression::Number(str.len() as f64)),
        _ => {
            let arg = check_type_onlyone!("length", &args[0], env, List)?;
            Ok(Expression::Number(arg.len() as f64))
        }
    }
}

pub fn func_str_eq(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("str.=", args, 2);
    args_len_max!("str.=", args, 2);
    let str1 = check_type_onlyone!("str.=", &args[0], env, String)?;
    let str2 = check_type_onlyone!("str.=", &args[1], env, String)?;

    Ok(Expression::Bool(str::eq(&str1, &str2)))
}

pub fn func_str_ne(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("str.!=", args, 2);
    args_len_max!("str.!=", args, 2);
    let str1 = check_type_onlyone!("str.!=", &args[0], env, String)?;
    let str2 = check_type_onlyone!("str.!=", &args[1], env, String)?;

    Ok(Expression::Bool(str::ne(&str1, &str2)))
}

pub fn func_str_lt(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("str.<", args, 2);
    args_len_max!("str.<", args, 2);
    let str1 = check_type_onlyone!("str.<", &args[0], env, String)?;
    let str2 = check_type_onlyone!("str.<", &args[1], env, String)?;

    Ok(Expression::Bool(str::lt(&str1, &str2)))
}

pub fn func_str_le(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("str.<=", args, 2);
    args_len_max!("str.<=", args, 2);
    let str1 = check_type_onlyone!("str.<=", &args[0], env, String)?;
    let str2 = check_type_onlyone!("str.<=", &args[1], env, String)?;

    Ok(Expression::Bool(str::le(&str1, &str2)))
}

pub fn func_str_gt(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("str.>", args, 2);
    args_len_max!("str.>", args, 2);
    let str1 = check_type_onlyone!("str.>", &args[0], env, String)?;
    let str2 = check_type_onlyone!("str.>", &args[1], env, String)?;

    Ok(Expression::Bool(str::gt(&str1, &str2)))
}

pub fn func_str_ge(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("str.>=", args, 2);
    args_len_max!("str.>=", args, 2);
    let str1 = check_type_onlyone!("str.>=", &args[0], env, String)?;
    let str2 = check_type_onlyone!("str.>=", &args[1], env, String)?;

    Ok(Expression::Bool(str::ge(&str1, &str2)))
}

pub fn func_last(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("last", args, 1);
    args_len_max!("last", args, 1);
    let str = check_type_onlyone!("last", &args[0], env, String)?;

    Ok(Expression::String(str[str.len() - 1..].to_string()))
}

pub fn func_chars(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("chars", args, 1);
    args_len_max!("chars", args, 1);
    let str = check_type_onlyone!("chars", &args[0], env, String)?;

    Ok(Expression::List(to_quote_list!(str
        .chars()
        .map(|x| Expression::String(x.to_string()))
        .collect::<Vec<_>>())))
}
pub fn func_find(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("find", args, 2);
    args_len_max!("find", args, 2);
    let str1 = check_type_onlyone!("find", &args[0], env, String)?;
    let str2 = check_type_onlyone!("find", &args[1], env, String)?;

    Ok(if let Some(a) = str1.find(&str2) {
        Expression::Number(a as f64)
    } else {
        Expression::Bool(false)
    })
}
pub fn func_contains(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("contains", args, 2);
    args_len_max!("contains", args, 2);
    let str1 = check_type_onlyone!("contains", &args[0], env, String)?;
    let str2 = check_type_onlyone!("contains", &args[1], env, String)?;

    Ok(Expression::Bool(str1.contains(&str2)))
}
pub fn func_insert(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("insert", args, 3);
    args_len_max!("insert", args, 3);
    let mut str1 = check_type_onlyone!("insert", &args[0], env, String)?;
    let num = check_type_onlyone!("insert", &args[1], env, Number)?;
    let str2 = check_type_onlyone!("insert", &args[2], env, String)?;
    str1.insert_str(num as usize, &str2);

    Ok(Expression::String(str1))
}

pub fn func_begin(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("begin", args, 1);
    args_len_max!("begin", args, 1);
    let str1 = check_type_onlyone!("begin", &args[0], env, String)?;

    Ok(if let Some(a) = str1.chars().next() {
        Expression::String(a.to_string())
    } else {
        Expression::Bool(false)
    })
}

pub fn func_is_empty(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("is-empty", args, 1);
    args_len_max!("is-empty", args, 1);
    let str1 = check_type_onlyone!("is-empty", &args[0], env, String)?;

    Ok(Expression::Bool(str1.is_empty()))
}

pub fn func_remove(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("remove", args, 2);
    args_len_max!("remove", args, 3);
    let mut str1 = check_type_onlyone!("remove", &args[0], env, String)?;
    let num1 = check_type_onlyone!("remove", &args[1], env, Number)? as usize;

    if args.len() == 2 {
        str1.remove(num1);
        Ok(Expression::String(str1))
    } else {
        let num2 = check_type_onlyone!("remove", &args[2], env, Number)? as usize;
        str1.drain(num1..num2 + 1);
        Ok(Expression::String(str1))
    }
}

pub fn func_reverse(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("reverse", args, 1);
    args_len_max!("reverse", args, 1);
    let str1 = check_type_onlyone!("reverse", &args[0], env, String)?;

    Ok(Expression::String(str1.chars().rev().collect::<String>()))
}

pub fn func_rfind(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("rfind", args, 2);
    args_len_max!("rfind", args, 2);
    let str1 = check_type_onlyone!("rfind", &args[0], env, String)?;
    let str2 = check_type_onlyone!("rfind", &args[1], env, String)?;

    Ok(if let Some(a) = str1.rfind(&str2) {
        Expression::Number(a as f64)
    } else {
        Expression::Bool(false)
    })
}

pub fn func_slice(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("slice", args, 3);
    args_len_max!("slice", args, 3);
    let str1 = check_type_onlyone!("slice", &args[0], env, String)?;
    let num1 = check_type_onlyone!("slice", &args[1], env, Number)? as usize;
    let num2 = check_type_onlyone!("slice", &args[2], env, Number)? as usize;

    if str1.len() <= num2 {
        return Err(GError::Reason(format!(
            "str.slice: index {} out of {}",
            num2,
            str1.len()
        )));
    }
    Ok(Expression::String(str1[num1..num2 + 1].to_string()))
}

pub fn func_str(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("str", args, 1);
    args_len_max!("str", args, 1);
    let meta = check_type_onlyone!("str", &args[0], env, String)?;

    Ok(Expression::String(
        meta.replace("\\b", " ").replace("\\n", "\n").replace("\\[", "(").replace("\\]", ")").replace("\\'", "\""),
    ))
}

pub fn func_str_plus(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("str.+", args, 2);
    args_len_max!("str.+", args, 2);
    let str1 = check_type_onlyone!("str.+", &args[0], env, String)?;
    let str2 = check_type_onlyone!("str.+", &args[1], env, String)?;

    Ok(Expression::String(str1 + &str2))
}

pub fn func_lines(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("lines", args, 1);
    args_len_max!("lines", args, 1);
    let str1 = check_type_onlyone!("lines", &args[0], env, String)?;

    Ok(Expression::List(
        str1.lines()
            .map(|a| Expression::String(a.to_owned()))
            .collect::<Vec<_>>(),
    ))
}