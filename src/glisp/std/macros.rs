/* Tiny Tiny Web
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */

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
pub(super) use args_len_max;

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
pub(super) use args_len_min;

macro_rules! check_type_onlyone {
    ($fnname:expr, $value:expr, $env:ident, $_type:ident, $config:expr) => {
        match eval($value, $env, $config) {
            Ok(Expression::$_type(a)) => Ok(a),
            _ => Err(GError::Reason(format!("{}: Unsupported type", $fnname))),
        }
    };
}
pub(super) use check_type_onlyone;

macro_rules! to_quote_list {
    ($list:expr) => {{
        let mut v = vec![Expression::Symbol("quote".to_string())];
        v.extend($list);
        v
    }};
}
pub(super) use to_quote_list;
