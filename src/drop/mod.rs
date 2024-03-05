/* Tiny-Tiny-Web/Drop
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License Version 3
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */

//! # Drop 综合功能库
//! ## http
//! HttpRequest: 可以解析任意标准的 HTTP 请求字符串
//! HttpResponse: 可以构造一个标准的 HTTP 响应字符串
//! 关于标准，参见[此文档](https://www.rfc-editor.org/rfc/rfc2616)
//!
//! ## log
//! 提供打印日志的方法，但通常需要进行二次封装
//! 至于如何二次封装，参见 log 函数的注释
//!
//! ## random
//! 生成随机数，基于微型梅森旋转(Tiny-MT)算法
//!
//! ## mempool
//! 一个内存池库，用以在高并发下避免过多的重新内存分配

pub mod base64;
pub mod http;
pub mod log;
pub mod mempool;
pub mod random;
pub mod thread;
pub mod time;
pub mod tool;
