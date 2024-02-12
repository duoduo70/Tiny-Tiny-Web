/* Tiny Tiny Web
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */

use super::macros::*;
use super::*;

pub fn serve(
    args: &[Expression],
    env: &mut Environment,
    config: Config,
) -> Result<Expression, GError> {
    args_len_min!("serve", args, 3);
    args_len_max!("serve", args, 3);

    let url = check_type_onlyone!("serve", &args[0], env, String, config.clone())?;
    let file_path = check_type_onlyone!("serve", &args[1], env, String, config.clone())?;
    let content_type = check_type_onlyone!("serve", &args[2], env, String, config.clone())?;

    if !std::path::Path::new(&("export/".to_owned() + &file_path)).is_file() {
        return Err(GError::Reason(
            "serve: The second arg is not a file".to_owned(),
        ));
    }

    if let Some(_config) = config {
        let mut _config = _config.borrow_mut();
        _config.router_config.serve_files_info.insert(
            url,
            crate::config::ServeFileData {
                file_path,
                content_type,
                replace: None,
            },
        );
        Ok(Expression::Bool(true))
    } else {
        Err(GError::Reason(
            "serve: This function is not supported in this mode".to_owned(),
        ))
    }
}
