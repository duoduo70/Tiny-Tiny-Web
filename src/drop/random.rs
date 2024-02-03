/* Tiny-Tiny-Web/Drop
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License Version 3
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */

use std::time::SystemTimeError;

use super::time::Time;

pub fn get_random_256() -> Result<(u128, u128), SystemTimeError> {
    let seed = Time::nsec()? as u128;
    let seed2 = Time::msec()? as u128;
    let low128 = (25214903917252149039190391712252147 * seed) & ((1 << 127) - 1) | 1; 
    let high128 = (23479875723479903917252112421248757 * seed2) & ((1 << 127) - 1) | 0;
    Ok((low128, high128))
}