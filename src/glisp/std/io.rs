/* Tiny Tiny Web
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */

use super::*;
use super::macros::*;

pub fn func_console_log(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("log", args, 1);
    args_len_max!("log", args, 1);

    let str1 = check_type_onlyone!("log", &args[0], env, String)?;

    use crate::drop::log::LogLevel::*;
    use crate::macros::*;

    log!(
        Info,
        format!(
            "[ghost-lisp] [console.log] {}",
            str1.replace("\\b", " ").replace("\\n", "\n")
        )
    );

    Ok(Expression::Bool(true))
}

pub fn func_read_file(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("read-file", args, 1);
    args_len_max!("read-file", args, 1);
    let filename = check_type_onlyone!("read-file", &args[0], env, String)?;

    if let Ok(a) = std::fs::read_to_string(filename) {
        Ok(Expression::String(a))
    } else {
        Err(GError::Reason("read-file: not a file".to_owned()))
    }
}

pub fn func_write_file(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
    args_len_min!("write-file", args, 2);
    args_len_max!("write-file", args, 2);
    let filename = check_type_onlyone!("write-file", &args[0], env, String)?;
    let str = check_type_onlyone!("write-file", &args[1], env, String)?;

    match std::fs::write(filename, str) {
        Ok(_) => Ok(Expression::Bool(true)),
        Err(_) => Ok(Expression::Bool(false)),
    }
}

pub fn func_read_dir(args: &[Expression], env: &mut Environment) -> Result<Expression, GError> {
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