/* Tiny-Tiny-Web
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License Version 3
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */

//! # 本模块的总则
//!
//! 本模块的作用有且仅有一个：让本项目支持 HTTPS
//!
//! # 关于 c25519 模块的特别说明
//! `c25519` 子模块的注释强制使用英文，因为它是独立的、需要国际化的项目
//! 应该以原项目作者的注释为主，在原作者的注释不清晰或不全面时，使用英文编写额外的注释
//! 该模块属于公有领域，可以不受限制的使用
//! 应该为该模块编写一些测试
//!
//! # 子模块
//! 
//! ## tls
//! 该模块是本模块的核心子模块，其增加了 TLS 传输协议的支持
//! TLS 协议是一个极为复杂的传输协议集合，涉及论文之多以至于无法在本总则中提及
//! 另请查看该模块之总则
//! 
//! ## ecc
//! 该模块为本模块增加 ECDSA 曲线相关算法的支持，在 TLS 协议中用来核对 CA 证书是否属于服务器
//! 
//! ## sha256
//! 该模块为本模块增加 SHA256 哈希算法的支持
//!
//! ## c25519
//! 该模块为本模块增加 c25519 曲线相关算法的支持，该模块和本项目的其它模块不同的是，其采用 CC0 协议
//! 这是一个加密、解密、签名、验签的算法，在 TLS 通信中起到重要作用
//! 在最新的 TLS 1.3 协议中，密钥交换算法仅支持 x25519 (基于 c25519 的算法，在本模块中同样有实现)
//! ed25519 (基于 c25519 的算法，在本模块中同样有实现) 是非标准的签名算法
//! ed25519 虽然不能直接在浏览器和服务器间的数据通信中使用，但它可以在部分服务器间的数据通信中使用
//! 因为它比标准方法更加快速和安全，所以，本项目选择将 ed25519 的支持作为目标的一部分
//! 转写自 C 语言 项目：https://github.com/DavyLandman/compact25519
//! 本 Rust 转写的 Github 仓库：https://github.com/duoduo70/Compact-C25519-rs
//! 原始的 Python 实现和该算法的相关论文，参见：https://www.dlbeer.co.nz/oss/c25519.html
//!

pub mod c25519;
pub mod ecc;
pub mod sha256;
pub mod tls;
