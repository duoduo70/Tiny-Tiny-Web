/* Tiny Tiny Web
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */

//! # 本模块的总则
//!
//! ## 关于 REPL
//!
//! 本程序内置了一个 REPL，它是 mode/toolmode.rs 的一部分
//! REPL的作用是方便调试、测试和学习本程序
//! 同时，这为本项目的子项目 vscode-ghost-lisp-extentsion 中的 Debug Console 支持做铺垫
//! 本模块的命名应该足够清晰和模块化，故不会有很多文档注释
//!
//! ## 关于本模块内注释的说明
//!
//! 对于大多数注解，会直接在总则中体现
//!
//! ## 关于 Ghost Lisp 编程语言
//!
//! Ghost Lisp 应该是一门基于 Lisp1 (弱类型) 的，可扩展的，基于列表的编程语言
//! 和很多 Lisp 方言不同，Ghost Lisp 没有装饰器之类概念，且没有陈述式的概念
//! 这意味着 Ghost Lisp 的任何一条语句都必须遵循如下结构，且没有例外：
//! ```lisp
//! (function-name arg1 args2 ...)
//! ```
//! 其中，参数是可选的
//! 对于其它语法，参见本项目的文档
//! 即：https://github.com/duoduo70/Tiny-Tiny-Web/blob/master/docs/index.md
//!
//! ## 关于为本模块增加或删改功能
//!
//! 请根据惯例，将功能注册到相关的文件中，如果没有合适的文件，可以另行创建
//! `core` 子模块定义了所有语法，`std` 子模块定义了所有内置的函数
//! 如果能通过增加内置函数的方法解决一个问题，就最好不要直接增加语法
//! 在本项目达到 Stable 阶段之后，最好不要删减或大改旧有功能

pub mod core;
pub mod repl;

mod std;
