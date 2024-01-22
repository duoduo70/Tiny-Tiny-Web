/* Tiny Tiny Web
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */
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
    "Can not select TCP listener to non-blocking mode",
    "Can not read a TCP Stream, the server may be about to crash.", // 27
    "Can not parse your address setting: ",
    "Unable to get time, There may be a serious failure within the OS.",
    "Unknown command", // 30
    "Invalid UTF-8 sequence",
    "ghost-lisp",
    "Return code:",
    "Compile error:"
);
