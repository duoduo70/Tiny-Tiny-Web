/* Tiny Tiny Web
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */

 #[cfg(not(feature = "chinese"))]
crate::macros::create_static_string_list!(
    LOG,
    "Tiny-Tiny-Web Started (Ver.",
    "Can not listen: ",
    "Can not open TCP Steam.",
    "Connection established: \n",
    "Connection handle: Can not read a buffer, was skipped.",
    "Malformed or unsupported request: ",
    "Connection handle: Can not write a buffer to the stream, was skipped.",
    "Connection request header: \n",
    "Connection response: \n",
    "Can not read configure file: ",
    "Can not parse configure: Syntax Error: ", //10
    "file:",
    "line:",
    "No routes are set.",
    "Router: Pushed a file: ",
    "Loading configure finished. ",
    "Void command. ",
    "Void item: ", //17
    "Not enough items. ",
    "This mapping does not exist. ",
    "This is not a file. ",
    "config-loader",
    "Can not read file: ", //22
    "Can not write to ",
    "Compiled successfully: ",
    "Can not import.", //25
    "Can not select TCP listener to non-blocking mode.",
    "Can not read a TCP Stream, the server may be about to crash.", // 27
    "Can not parse your address setting: ",
    "Unable to get time, There may be a serious failure within the OS.",
    "Unknown command", // 30
    "Invalid UTF-8 sequence",
    "ghost-lisp",
    "Return code:",
    "Error:", // 34
    "The pipe only receive string or bool, not: ",
    "Unsupported status code: "
);

#[cfg(feature = "chinese")]
crate::macros::create_static_string_list!(
    LOG,
    "Tiny-Tiny-Web 已启动 (版本 ",
    "无法监听: ",
    "无法打开 TCP 流.",
    "连接已建立: \n",
    "连接句柄：无法读取缓冲区，已被跳过.",
    "请求格式错误或不受支持: ",
    "连接句柄：无法将缓冲区写入流，已被跳过.",
    "连接请求头: \n",
    "连接响应: \n",
    "无法读取配置文件: ",
    "无法解析配置：语法错误: ", //10
    "文件:",
    "行:",
    "没有设置路由.",
    "路由：文件已推送: ",
    "加载配置完成. ",
    "未知命令. ",
    "未知项: ", //17
    "项不够. ",
    "该映射不存在. ",
    "这不是一个文件. ",
    "配置加载器",
    "无法读取文件: ", //22
    "无法写文件",
    "编译成功: ",
    "无法导入.", //25
    "无法设置 TCP 流为无缓冲模式.",
    "无法读取 TCP 流，服务器可能即将崩溃.", // 27
    "无法解析您的地址设置: ",
    "无法获取时间，操作系统内部可能已出现严重故障.",
    "未知命令", // 30
    "无效的 UTF-8 序列",
    "ghost-lisp",
    "返回码:",
    "错误:", // 34
    "Pipe 只接收字符串或布尔值，不接收: ",
    "不支持的状态码: "
);