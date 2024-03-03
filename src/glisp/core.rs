/* Tiny Tiny Web
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */

use super::std::eval_built_in_form;
use std::{collections::HashMap, fmt::Display, rc::Rc};

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
    pub params: Rc<Expression>,
    pub body: Rc<Expression>,
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
            Expression::Func(_) => write!(f, "function()"),
        }
    }
}

pub enum GError {
    Reason(String),
}

impl GError {
    pub fn into_string(self) -> Result<String, GError> {
        match self {
            GError::Reason(string) => Ok(string),
            #[allow(unreachable_patterns)]
            _ => Err(GError::Reason("Can not get GError reason".to_owned()))
        }
    }
}

pub struct Environment<'a> {
    pub data: HashMap<String, Expression>,
    pub outer: Option<&'a Environment<'a>>,
}

/// 使用 RefCell 包装是为了包装 `&'a mut crate::config::Config` 以使其可以被正确移动
/// 使用 Rc 是为了解决在递归式解析中不可避免的循环可变引用
/// 与其深拷贝一次 Config ，每次调用函数时多进行一次寻址在通常情况下可能更快
/// 并且，为了保持代码的一致性和可维护性，最终决定保留 `&mut` 引用
///
/// 在未来的版本中，如果`&mut crate::config::Config` 不足以支撑 crate::config 包，会考虑全部换成 RefCell
pub type Config<'a> = Option<Rc<std::cell::RefCell<&'a mut crate::config::Config>>>;

pub fn func_lambda(args: &[Expression]) -> Result<Expression, GError> {
    let params = args
        .first()
        .ok_or(GError::Reason("unexpected args form".to_string()))?;
    let body = args
        .get(1)
        .ok_or(GError::Reason("unexpected second form".to_string()))?;
    if args.len() != 2 {
        return Err(GError::Reason("lambda can only have two forms".to_string()));
    }
    Ok(Expression::Lambda(Lambda {
        params: Rc::new(params.clone()),
        body: Rc::new(body.clone()),
    }))
}

pub fn tokenize(expr: String) -> Vec<String> {
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
        .replace('(', " ( ")
        .replace(')', " ) ")
        .split_whitespace()
        .map(|x| x.to_string())
        .collect()
}

pub fn parse(tokens: &[String]) -> Result<(Expression, &[String]), GError> {
    let (token, rest) = tokens
        .split_first()
        .ok_or(GError::Reason("could not get token".to_string()))?;
    match &token[..] {
        "(" => read_seq(rest),
        ")" => Err(GError::Reason("unexpected `)`".to_string())),
        _ => Ok((parse_atom(token), rest)),
    }
}

pub fn read_seq(tokens: &[String]) -> Result<(Expression, &[String]), GError> {
    let mut res: Vec<Expression> = vec![];
    let mut xs = tokens;
    loop {
        let (next_token, rest) = xs
            .split_first()
            .ok_or(GError::Reason("could not find closing `)`".to_string()))?;
        if next_token == ")" {
            return Ok((Expression::List(res), rest));
        }
        let (exp, new_xs) = parse(xs)?;
        res.push(exp);
        xs = new_xs;
    }
}

pub fn parse_atom(token: &str) -> Expression {
    if token.len() >= 2
        && token.as_bytes()[0] == b'\"'
        && token.as_bytes()[token.len() - 1] == b'\"'
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
            if floats.first().is_none() || floats.get(1).is_none() {
                return Err(GError::Reason("expected number".to_string()));
            }
            let is_ok = floats.first().unwrap().eq(floats.get(1).unwrap());
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
                        #[allow(clippy::redundant_closure_call)]
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

pub fn eval(exp: &Expression, env: &mut Environment, config: Config) -> Result<Expression, GError> {
    match exp {
        Expression::Bool(_) => Ok(exp.clone()),
        Expression::Symbol(k) => env_get(k, env)
            .ok_or(GError::Reason(format!("unexpected symbol k={}", k))),
        Expression::Number(_a) => Ok(exp.clone()),
        Expression::List(list) => {
            let first_form = list
                .first()
                .ok_or(GError::Reason("expected a non-empty list".to_string()))?;
            let arg_forms = &list[1..];

            match eval_built_in_form(first_form, arg_forms, env, config.clone()) {
                Some(built_in_res) => built_in_res,
                None => {
                    let first_eval = eval(first_form, env, config.clone())?;
                    match first_eval {
                        Expression::Func(f) => {
                            let args_eval = arg_forms
                                .iter()
                                .map(|x| eval(x, env, config.clone()))
                                .collect::<Result<Vec<Expression>, GError>>();
                            f(&args_eval?)
                        }
                        Expression::Lambda(lambda) => {
                            // ->  New
                            let new_env =
                                &mut env_for_lambda(lambda.params, arg_forms, env, config.clone())?;
                            eval(&lambda.body, new_env, config)
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

pub fn func_if(
    args: &[Expression],
    env: &mut Environment,
    config: Config,
) -> Result<Expression, GError> {
    let test_form = args
        .first()
        .ok_or(GError::Reason("expected test form".to_string()))?;
    let test_eval = eval(test_form, env, config.clone())?;
    match test_eval {
        Expression::Bool(b) => {
            let form_idx = if b { 1 } else { 2 };
            let res_form = args
                .get(form_idx)
                .ok_or(GError::Reason(format!("expected form idx={}", form_idx)))?;
            eval(res_form, env, config)
        }
        _ => Err(GError::Reason(format!(
            "unexpected test form='{}'",
            test_form
        ))),
    }
}

pub fn parse_eval(
    expr: String,
    env: &mut Environment,
    config: Config,
) -> Result<Expression, GError> {
    let (parsed_exp, _) = parse(&tokenize(expr))?;
    let evaled_exp = eval(&parsed_exp, env, config)?;
    Ok(evaled_exp)
}

pub fn slurp_expr() -> String {
    let mut expr = String::new();
    std::io::stdin()
        .read_line(&mut expr)
        .expect("Failed to read line");
    expr
}

fn parse_list_of_floats(args: &[Expression]) -> Result<Vec<f64>, GError> {
    args.iter().map(parse_single_float).collect()
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
    config: Config,
) -> Result<Environment<'a>, GError> {
    let ks = parse_list_of_symbol_strings(params)?;
    if ks.len() != args.len() {
        return Err(GError::Reason(format!(
            "expected {} params, got {}",
            ks.len(),
            args.len()
        )));
    }
    let vs = eval_forms(args, outer_env, config)?;
    let mut data: HashMap<String, Expression> = HashMap::new();
    for (k, v) in ks.iter().zip(vs.iter()) {
        data.insert(k.clone(), v.clone());
    }

    Ok(Environment {
        data,
        outer: Some(outer_env),
    })
}

fn eval_forms(
    args: &[Expression],
    env: &mut Environment,
    config: Config,
) -> Result<Vec<Expression>, GError> {
    args.iter().map(|x| eval(x, env, config.clone())).collect()
}

fn parse_list_of_symbol_strings(params: Rc<Expression>) -> Result<Vec<String>, GError> {
    let list = match params.as_ref() {
        Expression::List(s) => Ok(s.clone()),
        _ => Err(GError::Reason("expected params to be a list".to_string())),
    }?;
    list.iter()
        .map(|x| match x {
            Expression::Symbol(s) => Ok(s.clone()),
            _ => Err(GError::Reason(
                "expected symbol in the argument list".to_string(),
            )),
        })
        .collect()
}

fn env_get(key: &str, env: &Environment) -> Option<Expression> {
    match env.data.get(key) {
        Some(exp) => Some(exp.clone()),
        None => match env.outer {
            Some(outer_env) => env_get(key, outer_env),
            None => None,
        },
    }
}
