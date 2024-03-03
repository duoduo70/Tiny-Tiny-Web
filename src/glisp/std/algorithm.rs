/* Tiny Tiny Web
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */

use super::macros::*;
 use super::*;

 pub fn func_map(
    args: &[Expression],
    env: &mut Environment,
    config: Config,
) -> Result<Expression, GError> {
    args_len_min!("map", args, 2);
    args_len_max!("map", args, 2);

    let fst = check_type_onlyone!("map", &args[0], env, Func, config.clone())?;
    let snd = check_type_onlyone!("map", &args[0], env, List, config.clone())?;

    let mut vec = vec![];
    let mut i = 1;
    for element in snd {
        match fst(&[element]) {
            Ok(element) => {
                vec.push(element);
            }
            Err(error) => {
                if let Ok(string) = error.into_string() {
                    return Err(GError::Reason("map: Error while mapping ".to_owned()+&i.to_string()+"th element: " + &string))
                } else {
                    return Err(GError::Reason("map: Error while mapping ".to_owned()+&i.to_string()+"th element and cannot get the error message"))
                }
            }
        }
        i+=1;
    }

    Ok(Expression::List(vec))
}