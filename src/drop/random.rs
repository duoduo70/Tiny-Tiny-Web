/* Tiny-Tiny-Web/Drop
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License Version 3
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */

use std::time::SystemTimeError;

use super::time::Time;

/// 生成一个 256 位的随机数，用两段 128 位字节表示
/// 这个随机数生成器仅用作开发，未来必须重写
/// FIXME: 重新选择随机数，或推翻重写，以修复随机数不够随机的问题
pub fn get_random_256() -> Result<(u128, u128), SystemTimeError> {
    let seed = Time::nsec()? as u128;
    let seed2 = Time::msec()? as u128;
    #[allow(clippy::identity_op)]
    let low128 = (25214903917252149039190391712252147 * seed) & ((1 << 127) - 1) | 1;
    #[allow(clippy::identity_op)]
    let high128 = (23479875723479903917252112421248757 * seed2) & ((1 << 127) - 1) | 0;
    Ok((low128, high128))
}
