/* Tiny Tiny Web
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */
use std::{collections::HashMap, fmt::Display, io::Write, rc::Rc};

//TODO: Standardize error reporting
//TODO: i18n support

#[derive(Clone, PartialEq)]
pub enum Expression {
    Symbol(String),
    Number(f64),
    List(Vec<Expression>),
    Func(fn(&[Expression]) -> Result<Expression, GError>),
    Bool(bool),
    Lambda(Lambda),
    String(String),
}

#[derive(Clone, PartialEq)]
pub struct Lambda {
    params: Rc<Expression>,
    body: Rc<Expression>,
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Symbol(a) => write!(f, "{}", a),
            Expression::Number(a) => write!(f, "{}", a),
            Expression::List(a) => write!(
                f,
                "{:?}",
                a.iter().map(|e| { format!("{}", e) }).collect::<Vec<_>>()
            ),
            Expression::Bool(a) => write!(f, "{}", a),
            Expression::Lambda(a) => {
                write!(f, "lambda: {{ params: {} , body: {} }}", &a.params, &a.body)
            }
            Expression::String(a) => write!(f, "\"{}\"", a),
            Expression::Func(_) => write!(f, "function()"), // TODO: Display function sign
        }
    }
}

pub enum GError {
    Reason(String),
}

pub struct Environment<'a> {
    data: HashMap<String, Expression>,
    outer: Option<&'a Environment<'a>>,
}

fn eval_lambda_args(args: &[Expression]) -> Result<Expression, GError> {
    let params = args
        .first()
        .ok_or(GError::Reason(format!("unexpected args form")))?;
    let body = args
        .get(1)
        .ok_or(GError::Reason(format!("unexpected second form")))?;
    if args.len() != 2 {
        return Err(GError::Reason(format!("lambda can only have two forms")));
    }
    Ok(Expression::Lambda(Lambda {
        params: Rc::new(params.clone()),
        body: Rc::new(body.clone()),
    }))
}

fn tokenize(expr: String) -> Vec<String> {
    let lines = expr.lines();
    let mut new_expr = String::new();
    for line in lines {
        if let Some(a) = line.find(';') {
            new_expr += &line[..a]
        } else {
            new_expr += line
        }
    }
    new_expr
        .replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|x| x.to_string())
        .collect()
}

fn parse<'a>(tokens: &'a [String]) -> Result<(Expression, &'a [String]), GError> {
    let (token, rest) = tokens
        .split_first()
        .ok_or(GError::Reason("could not get token".to_string()))?;
    match &token[..] {
        "(" => read_seq(rest),
        ")" => Err(GError::Reason("unexpected `)`".to_string())),
        _ => Ok((parse_atom(token), rest)),
    }
}

fn read_seq<'a>(tokens: &'a [String]) -> Result<(Expression, &'a [String]), GError> {
    let mut res: Vec<Expression> = vec![];
    let mut xs = tokens;
    loop {
        let (next_token, rest) = xs
            .split_first()
            .ok_or(GError::Reason("could not find closing `)`".to_string()))?;
        if next_token == ")" {
            return Ok((Expression::List(res), rest));
        }
        let (exp, new_xs) = parse(&xs)?;
        res.push(exp);
        xs = new_xs;
    }
}

fn parse_atom(token: &str) -> Expression {
    if token.len() >= 2
        && token.bytes().nth(0).unwrap() == b'\"'
        && token.bytes().nth(token.len() - 1).unwrap() == b'\"'
    {
        return Expression::String(token[1..token.len() - 1].to_string());
    }
    match token {
        "true" => Expression::Bool(true),
        "false" => Expression::Bool(false),
        _ => {
            let potential_float = token.parse();
            match potential_float {
                Ok(v) => Expression::Number(v),
                Err(_) => Expression::Symbol(token.to_string().clone()),
            }
        }
    }
}

pub fn default_env<'a>() -> Environment<'a> {
    let mut data: HashMap<String, Expression> = HashMap::new();
    data.insert(
        "+".to_string(),
        Expression::Func(|args: &[Expression]| -> Result<Expression, GError> {
            let sum = parse_list_of_floats(args)?
                .iter()
                .fold(0.0, |sum, a| sum + a);
            Ok(Expression::Number(sum))
        }),
    );
    data.insert(
        "-".to_string(),
        Expression::Func(|args: &[Expression]| -> Result<Expression, GError> {
            let floats = parse_list_of_floats(args)?;
            let first = *floats
                .first()
                .ok_or(GError::Reason("expected at least one number".to_string()))?;
            let sum_of_rest = floats[1..].iter().fold(0.0, |sum, a| sum + a);

            Ok(Expression::Number(first - sum_of_rest))
        }),
    );

    data.insert(
        "=".to_string(),
        Expression::Func(|args: &[Expression]| -> Result<Expression, GError> {
            let floats = parse_list_of_floats(args)?;
            // 要想比较，需要有两个值
            if floats.len() != 2 {
                return Err(GError::Reason("expected two number".to_string()));
            }
            // 将第 0 个元素和第 1 个元素进行比较
            if floats.get(0).is_none() || floats.get(1).is_none() {
                return Err(GError::Reason("expected number".to_string()));
            }
            let is_ok = floats.get(0).unwrap().eq(floats.get(1).unwrap());
            Ok(Expression::Bool(is_ok))
        }),
    );

    macro_rules! ensure_tonicity {
        ($check_fn:expr) => {{
            |args: &[Expression]| -> Result<Expression, GError> {
                let floats = parse_list_of_floats(args)?;
                let first = floats
                    .first()
                    .ok_or(GError::Reason("expected at least one number".to_string()))?;
                let rest = &floats[1..];
                fn f(prev: &f64, xs: &[f64]) -> bool {
                    match xs.first() {
                        Some(x) => $check_fn(prev, x) && f(x, &xs[1..]),
                        None => true,
                    }
                }
                Ok(Expression::Bool(f(first, rest)))
            }
        }};
    }

    data.insert(
        ">".to_string(),
        Expression::Func(ensure_tonicity!(|a, b| a > b)),
    );

    data.insert(
        "<".to_string(),
        Expression::Func(ensure_tonicity!(|a, b| a < b)),
    );

    data.insert(
        "<=".to_string(),
        Expression::Func(ensure_tonicity!(|a, b| a <= b)),
    );

    data.insert(
        ">=".to_string(),
        Expression::Func(ensure_tonicity!(|a, b| a >= b)),
    );

    Environment { data, outer: None }
}

fn parse_list_of_floats(args: &[Expression]) -> Result<Vec<f64>, GError> {
    args.iter().map(|x| parse_single_float(x)).collect()
}

fn parse_single_float(exp: &Expression) -> Result<f64, GError> {
    match exp {
        Expression::Number(num) => Ok(*num),
        Expression::Bool(b) => Ok((*b).into()),
        _ => Err(GError::Reason("expect a number".to_string())),
    }
}

fn env_for_lambda<'a>(
    params: Rc<Expression>,
    args: &[Expression],
    outer_env: &'a mut Environment,
) -> Result<Environment<'a>, GError> {
    let ks = parse_list_of_symbol_strings(params)?;
    if ks.len() != args.len() {
        return Err(GError::Reason(format!(
            "expected {} params, got {}",
            ks.len(),
            args.len()
        )));
    }
    let vs = eval_forms(args, outer_env)?;
    let mut data: HashMap<String, Expression> = HashMap::new();
    for (k, v) in ks.iter().zip(vs.iter()) {
        data.insert(k.clone(), v.clone());
    }

    Ok(Environment {
        data,
        outer: Some(outer_env),
    })
}

fn eval_forms(args: &[Expression], env: &mut Environment) -> Result<Vec<Expression>, GError> {
    args.iter().map(|x| eval(x, env)).collect()
}

fn parse_list_of_symbol_strings(params: Rc<Expression>) -> Result<Vec<String>, GError> {
    let list = match params.as_ref() {
        Expression::List(s) => Ok(s.clone()),
        _ => Err(GError::Reason(format!("expected params to be a list"))),
    }?;
    list.iter()
        .map(|x| match x {
            Expression::Symbol(s) => Ok(s.clone()),
            _ => Err(GError::Reason(format!(
                "expected symbol in the argument list"
            ))),
        })
        .collect()
}

fn env_get(key: &str, env: &Environment) -> Option<Expression> {
    match env.data.get(key) {
        Some(exp) => Some(exp.clone()),
        None => match env.outer {
            Some(outer_env) => env_get(key, &outer_env),
            None => None,
        },
    }
}

fn eval(exp: &Expression, env: &mut Environment) -> Result<Expression, GError> {
    match exp {
        Expression::Bool(_) => Ok(exp.clone()),
        Expression::Symbol(k) => env_get(&k, env)
            .ok_or(GError::Reason(format!("unexpected symbol k={}", k)))
            .map(|x| x.clone()),
        Expression::Number(_a) => Ok(exp.clone()),
        Expression::List(list) => {
            let first_form = list
                .first()
                .ok_or(GError::Reason("expected a non-empty list".to_string()))?;
            let arg_forms = &list[1..];

            match eval_built_in_form(first_form, arg_forms, env) {
                Some(built_in_res) => built_in_res,
                None => {
                    let first_eval = eval(first_form, env)?;
                    match first_eval {
                        Expression::Func(f) => {
                            let args_eval = arg_forms
                                .iter()
                                .map(|x| eval(x, env))
                                .collect::<Result<Vec<Expression>, GError>>();
                            f(&args_eval?)
                        }
                        Expression::Lambda(lambda) => {
                            // ->  New
                            let new_env = &mut env_for_lambda(lambda.params, arg_forms, env)?;
                            eval(&lambda.body, new_env)
                        }
                        _ => Err(GError::Reason("first form must be a function".to_string())),
                    }
                }
            }
        }
        Expression::Func(_) => Err(GError::Reason("unexpected form".to_string())),
        Expression::String(_) => Ok(exp.clone()),
        _ => Err(GError::Reason("not supported type.".to_string())),
    }
}

fn eval_if_args(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    let test_form = args
        .first()
        .ok_or(GError::Reason("expected test form".to_string()))?;
    let test_eval = eval(test_form, env)?;
    match test_eval {
        Expression::Bool(b) => {
            let form_idx = if b { 1 } else { 2 };
            let res_form = args
                .get(form_idx)
                .ok_or(GError::Reason(format!("expected form idx={}", form_idx)))?;
            let res_eval = eval(res_form, env);
            res_eval
        }
        _ => Err(GError::Reason(format!(
            "unexpected test form='{}'",
            test_form.to_string()
        ))),
    }
}

pub fn parse_eval(expr: String, env: &mut Environment) -> Result<Expression, GError> {
    let (parsed_exp, _) = parse(&tokenize(expr))?;
    let evaled_exp = eval(&parsed_exp, env)?;
    Ok(evaled_exp)
}

fn slurp_expr() -> String {
    let mut expr = String::new();
    std::io::stdin()
        .read_line(&mut expr)
        .expect("Failed to read line");
    expr
}

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

fn eval_quote_args(args: &[Expression]) -> Result<Expression, GError> {
    let _fst = args
        .first()
        .ok_or(GError::Reason(format!("unexpected args form")))?;
    let mut retfst = vec![Expression::Symbol("quote".to_owned())];
    retfst.extend_from_slice(args);
    Ok(Expression::List(retfst))
}

fn eval_atom_args(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
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
fn eval_eq_args(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
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

macro_rules! args_len_max {
    ($fnname:expr, $args:expr, $ident:expr) => {
        if $args.len() > $ident {
            return Err(GError::Reason(format!(
                "{}: There are more parameters than the maximum {} allowed",
                std::stringify!($fnname),
                $ident
            )));
        }
    };
}

macro_rules! args_len_min {
    ($fnname:expr, $args:expr, $ident:expr) => {
        if $args.len() < $ident {
            return Err(GError::Reason(format!(
                "{}: There are more parameters than the minimum {} allowed",
                std::stringify!($fnname),
                $ident
            )));
        }
    };
}

fn eval_cons_args(args: &[Expression]) -> Result<Expression, GError> {
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

fn eval_cond_args(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
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

macro_rules! check_type_onlyone {
    ($fnname:expr, $value:expr, $env:ident, $_type:ident) => {
        match eval($value, $env) {
            Ok(Expression::$_type(a)) => Ok(a),
            _ => Err(GError::Reason(format!("{}: Unsupported type", $fnname))),
        }
    };
}

fn eval_set_args(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
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

fn eval_car_args(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("car", args, 1);
    args_len_max!("car", args, 1);
    let list = check_type_onlyone!("car", &args[0], env, List)?;
    if list.len() == 0 {
        Ok(Expression::List(vec![]))
    } else {
        Ok(list[0].clone())
    }
}

fn eval_cdr_args(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("cdr", args, 1);
    args_len_max!("cdr", args, 1);
    let list = check_type_onlyone!("cdr", &args[0], env, List)?;
    if list.len() <= 1 {
        Ok(Expression::List(vec![]))
    } else {
        Ok(Expression::List(list[1..].to_vec()))
    }
}

fn eval_length_args(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
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

fn eval_str_eq_args(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("str.=", args, 2);
    args_len_max!("str.=", args, 2);
    let str1 = check_type_onlyone!("str.=", &args[0], env, String)?;
    let str2 = check_type_onlyone!("str.=", &args[1], env, String)?;

    Ok(Expression::Bool(str::eq(&str1, &str2)))
}

fn eval_str_ne_args(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("str.!=", args, 2);
    args_len_max!("str.!=", args, 2);
    let str1 = check_type_onlyone!("str.!=", &args[0], env, String)?;
    let str2 = check_type_onlyone!("str.!=", &args[1], env, String)?;

    Ok(Expression::Bool(str::ne(&str1, &str2)))
}
fn eval_str_lt_args(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("str.<", args, 2);
    args_len_max!("str.<", args, 2);
    let str1 = check_type_onlyone!("str.<", &args[0], env, String)?;
    let str2 = check_type_onlyone!("str.<", &args[1], env, String)?;

    Ok(Expression::Bool(str::lt(&str1, &str2)))
}
fn eval_str_le_args(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("str.<=", args, 2);
    args_len_max!("str.<=", args, 2);
    let str1 = check_type_onlyone!("str.<=", &args[0], env, String)?;
    let str2 = check_type_onlyone!("str.<=", &args[1], env, String)?;

    Ok(Expression::Bool(str::le(&str1, &str2)))
}
fn eval_str_gt_args(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("str.>", args, 2);
    args_len_max!("str.>", args, 2);
    let str1 = check_type_onlyone!("str.>", &args[0], env, String)?;
    let str2 = check_type_onlyone!("str.>", &args[1], env, String)?;

    Ok(Expression::Bool(str::gt(&str1, &str2)))
}
fn eval_str_ge_args(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("str.>=", args, 2);
    args_len_max!("str.>=", args, 2);
    let str1 = check_type_onlyone!("str.>=", &args[0], env, String)?;
    let str2 = check_type_onlyone!("str.>=", &args[1], env, String)?;

    Ok(Expression::Bool(str::ge(&str1, &str2)))
}
fn eval_last_args(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("last", args, 1);
    args_len_max!("last", args, 1);
    let str = check_type_onlyone!("last", &args[0], env, String)?;

    Ok(Expression::String(str[str.len() - 1..].to_string()))
}
macro_rules! to_quote_list {
    ($list:expr) => {{
        let mut v = vec![Expression::Symbol("quote".to_string())];
        v.extend($list);
        v
    }};
}
fn eval_chars_args(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("chars", args, 1);
    args_len_max!("chars", args, 1);
    let str = check_type_onlyone!("chars", &args[0], env, String)?;

    Ok(Expression::List(to_quote_list!(str
        .chars()
        .map(|x| Expression::String(x.to_string()))
        .collect::<Vec<_>>())))
}
fn eval_find_args(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
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
fn eval_contains_args(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("contains", args, 2);
    args_len_max!("contains", args, 2);
    let str1 = check_type_onlyone!("contains", &args[0], env, String)?;
    let str2 = check_type_onlyone!("contains", &args[1], env, String)?;

    Ok(Expression::Bool(str1.contains(&str2)))
}
fn eval_insert_args(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("insert", args, 3);
    args_len_max!("insert", args, 3);
    let mut str1 = check_type_onlyone!("insert", &args[0], env, String)?;
    let num = check_type_onlyone!("insert", &args[1], env, Number)?;
    let str2 = check_type_onlyone!("insert", &args[2], env, String)?;
    str1.insert_str(num as usize, &str2);

    Ok(Expression::String(str1))
}

fn eval_begin_args(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("begin", args, 1);
    args_len_max!("begin", args, 1);
    let str1 = check_type_onlyone!("begin", &args[0], env, String)?;

    Ok(if let Some(a) = str1.chars().next() {
        Expression::String(a.to_string())
    } else {
        Expression::Bool(false)
    })
}

fn eval_is_empty_args(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("is-empty", args, 1);
    args_len_max!("is-empty", args, 1);
    let str1 = check_type_onlyone!("is-empty", &args[0], env, String)?;

    Ok(Expression::Bool(str1.is_empty()))
}
fn eval_remove_args(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
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

fn eval_reverse_args(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("reverse", args, 1);
    args_len_max!("reverse", args, 1);
    let str1 = check_type_onlyone!("reverse", &args[0], env, String)?;

    Ok(Expression::String(str1.chars().rev().collect::<String>()))
}

fn eval_rfind_args(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
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

fn eval_slice_args(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
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

fn eval_console_log_args(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("log", args, 1);
    args_len_max!("log", args, 1);

    let str1 = check_type_onlyone!("log", &args[0], env, String)?;

    use super::super::drop::log::LogLevel::*;
    use super::super::marco::*;

    log!(
        Info,
        format!(
            "[ghost-lisp] [console.log] {}",
            str1.replace("\\b", " ").replace("\\n", "\n")
        )
    );

    Ok(Expression::Bool(true))
}

fn eval_loop_args(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
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

fn eval_read_file_args(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("read-file", args, 1);
    args_len_max!("read-file", args, 1);
    let filename = check_type_onlyone!("read-file", &args[0], env, String)?;

    if let Ok(a) = std::fs::read_to_string(filename) {
        Ok(Expression::String(a))
    } else {
        Err(GError::Reason("read-file: not a file".to_owned()))
    }
}

fn eval_write_file_args(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("write-file", args, 2);
    args_len_max!("write-file", args, 2);
    let filename = check_type_onlyone!("write-file", &args[0], env, String)?;
    let str = check_type_onlyone!("write-file", &args[1], env, String)?;

    match std::fs::write(filename, str) {
        Ok(_) => Ok(Expression::Bool(true)),
        Err(_) => Ok(Expression::Bool(false)),
    }
}

fn eval_do_args(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("do", args, 1);

    let ret: &mut Expression = &mut Expression::List(vec![]);
    for e in args {
        *ret = eval(e, env)?;
    }
    Ok(ret.clone())
}

fn eval_meta_args(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("meta", args, 1);
    args_len_max!("meta", args, 1);
    let code = eval(&args[0], env)?;

    Ok(Expression::String(code.to_string()))
}

fn eval_eval_atom_args(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("eval-atom", args, 1);
    args_len_max!("eval-atom", args, 1);
    let meta = check_type_onlyone!("eval-atom", &args[0], env, String)?;

    eval(&parse_atom(&meta), env)
}

fn eval_str_args(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("str", args, 1);
    args_len_max!("str", args, 1);
    let meta = check_type_onlyone!("str", &args[0], env, String)?;

    Ok(Expression::String(
        meta.replace("\\b", " ").replace("\\n", "\n").replace("\\[", "(").replace("\\]", ")").replace("\\'", "\""),
    ))
}

fn eval_str_plus_args(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("str.+", args, 2);
    args_len_max!("str.+", args, 2);
    let str1 = check_type_onlyone!("str.+", &args[0], env, String)?;
    let str2 = check_type_onlyone!("str.+", &args[1], env, String)?;

    Ok(Expression::String(str1 + &str2))
}

fn eval_or_args(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("or", args, 2);
    args_len_max!("or", args, 2);
    let bool1 = check_type_onlyone!("or", &args[0], env, Bool)?;
    let bool2 = check_type_onlyone!("or", &args[1], env, Bool)?;

    Ok(Expression::Bool(bool1 || bool2))
}

fn eval_and_args(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("and", args, 2);
    args_len_max!("and", args, 2);
    let bool1 = check_type_onlyone!("and", &args[0], env, Bool)?;
    if !bool1 {
        return Ok(Expression::Bool(false));
    }
    let bool2 = check_type_onlyone!("and", &args[1], env, Bool)?;

    Ok(Expression::Bool(bool2))
}

fn eval_lines_args(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("lines", args, 1);
    args_len_max!("lines", args, 1);
    let str1 = check_type_onlyone!("lines", &args[0], env, String)?;

    Ok(Expression::List(
        str1.lines()
            .map(|a| Expression::String(a.to_owned()))
            .collect::<Vec<_>>(),
    ))
}

fn eval_read_dir_args(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("read-dir", args, 1);
    args_len_max!("read-dir", args, 1);
    let str1 = check_type_onlyone!("read-dir", &args[0], env, String)?;
    let dir = std::fs::read_dir(str1);
    match dir {
        Ok(readdir) => {
            let mut v = vec![];
            for e in readdir {
                if let Ok(a) = e {
                    v.push(Expression::String(
                        a.file_name().to_string_lossy().to_string(),
                    ));
                }
            }
            Ok(Expression::List(v))
        }
        _ => Ok(Expression::Bool(false)),
    }
}

fn eval_for_each_eval_args(
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

fn eval_eval_args(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("eval", args, 1);
    args_len_max!("eval", args, 1);
    let meta = check_type_onlyone!("eval", &args[0], env, String)?;

    let (parsed_exp, _) = parse(&tokenize(meta))?;
    Ok(eval(&parsed_exp, env)?)
}

fn eval_built_in_form(
    exp: &Expression,
    other_args: &[Expression],
    env: &mut Environment,
) -> Option<Result<Expression, GError>> {
    match exp {
        Expression::Symbol(symbol) => match symbol.as_ref() {
            "if" => Some(eval_if_args(other_args, env)),
            "set" => Some(eval_set_args(other_args, env)),
            "lambda" => Some(eval_lambda_args(other_args)),
            "quote" => Some(eval_quote_args(other_args)),
            "atom" => Some(eval_atom_args(other_args, env)),
            "eq" => Some(eval_eq_args(other_args, env)),
            "car" => Some(eval_car_args(other_args, env)),
            "cdr" => Some(eval_cdr_args(other_args, env)),
            "cons" => Some(eval_cons_args(other_args)),
            "cond" => Some(eval_cond_args(other_args, env)),
            "length" => Some(eval_length_args(other_args, env)),
            "str.=" => Some(eval_str_eq_args(other_args, env)),
            "str.!=" => Some(eval_str_ne_args(other_args, env)),
            "str.<" => Some(eval_str_lt_args(other_args, env)),
            "str.<=" => Some(eval_str_le_args(other_args, env)),
            "str.>" => Some(eval_str_gt_args(other_args, env)),
            "str.>=" => Some(eval_str_ge_args(other_args, env)),
            "last" => Some(eval_last_args(other_args, env)),
            "chars" => Some(eval_chars_args(other_args, env)),
            "find" => Some(eval_find_args(other_args, env)),
            "contains" => Some(eval_contains_args(other_args, env)),
            "insert" => Some(eval_insert_args(other_args, env)),
            "begin" => Some(eval_begin_args(other_args, env)),
            "is-empty" => Some(eval_is_empty_args(other_args, env)),
            "remove" => Some(eval_remove_args(other_args, env)),
            "reverse" => Some(eval_reverse_args(other_args, env)),
            "rfind" => Some(eval_rfind_args(other_args, env)),
            "slice" => Some(eval_slice_args(other_args, env)),
            "log" => Some(eval_console_log_args(other_args, env)),
            "loop" => Some(eval_loop_args(other_args, env)),
            "read-file" => Some(eval_read_file_args(other_args, env)),
            "write-file" => Some(eval_write_file_args(other_args, env)),
            "do" => Some(eval_do_args(other_args, env)),
            "meta" => Some(eval_meta_args(other_args, env)),
            "eval-atom" => Some(eval_eval_atom_args(other_args, env)),
            "str" => Some(eval_str_args(other_args, env)),
            "str.+" => Some(eval_str_plus_args(other_args, env)),
            "or" => Some(eval_or_args(other_args, env)),
            "and" => Some(eval_and_args(other_args, env)),
            "lines" => Some(eval_lines_args(other_args, env)),
            "return" => Some(Ok(Expression::Symbol("return".to_owned()))),
            "continue" => Some(Ok(Expression::Symbol("continue".to_owned()))),
            "pass" => Some(Ok(Expression::Symbol("pass".to_owned()))),
            "read-dir" => Some(eval_read_dir_args(other_args, env)),
            "for-each-eval" => Some(eval_for_each_eval_args(other_args, env)),
            "eval" => Some(eval_eval_args(other_args, env)),
            _ => None,
        },
        _ => None,
    }
}
