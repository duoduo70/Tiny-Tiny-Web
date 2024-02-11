/* Tiny-Tiny-Web/Drop
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License Version 3
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */
use std::thread::{spawn, JoinHandle};

/// 创建一个线程池，你永远不用在乎结构体内是什么
pub struct ThreadPool {
    joinhandles: Vec<JoinHandle<()>>,
}
impl ThreadPool {
    pub fn new() -> Self {
        ThreadPool {
            joinhandles: Vec::new(),
        }
    }
    /// stop_if_num_gt: 如果线程池内的线程数超过给定值，则等待
    /// 直到线程池内的线程数小于给定值，然后继续运行本线程
    /// func: 函数或闭包，通常来讲必须是全局的或在 main 函数中被定义
    /// 除非使用了发散函数的技巧
    pub fn add(
        &mut self,
        stop_if_num_gt: usize,
        func: impl FnOnce() + std::marker::Send + 'static,
    ) {
        if self.joinhandles.len() == stop_if_num_gt + 1 {
            self.joinhandles.remove(0).join().unwrap();
        }
        self.joinhandles.push(spawn(func));
    }
}
