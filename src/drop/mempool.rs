/* Tiny-Tiny-Web/Drop
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License Version 3
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */

//! WIP

use std::sync::atomic::AtomicU16;

/// 一个标号，它应该是群
/// $$ ({0, 1, 2 ,..., #THREADS_NUM - 1}, +) $$
/// 上的元素。
///
/// 简单的说，当该标号达到 THREADS_NUM 时，会归零。
///
/// 它指示了当前即将被生成的线程要利用哪个 MemModel 。
pub static CURRENT_THREAD_NUM: AtomicU16 = AtomicU16::new(0);

pub struct HttpRequestMemModel {
    req_str: String
}

#[cfg(test)]
mod test {
    #[test]
    fn test(){
        
    }
}