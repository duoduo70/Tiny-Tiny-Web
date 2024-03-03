/* Tiny-Tiny-Web/Drop
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License Version 3
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */
#[cfg(feature = "nightly")]
pub fn decode_unchecked(base64str: &str) -> Vec<u8> {
    let mut decoded = Vec::new(); // 存储解码后的字节
    
    let mut buffer: u32 = 0; // 用于缓存解码过程中的数据
    let mut buffer_size = 0; // 缓冲区的大小
    
    for c in base64str.chars() {
        let value = match c {
            'A'..='Z' => c as u32 - 'A' as u32,
            'a'..='z' => c as u32 - 'a' as u32 + 26,
            '0'..='9' => c as u32 - '0' as u32 + 52,
            '+' => 62,
            '/' => 63,
            '=' => break, // 遇到'='表示Base64字符串结束
            _ => continue, // 忽略非Base64字符
        };
        
        buffer = (buffer << 6) | value; // 将当前字符的值添加到缓冲区
        
        buffer_size += 6; // 增加缓冲区的大小
        
        if buffer_size >= 8 {
            buffer_size -= 8;
            let byte = (buffer >> buffer_size) as u8; // 从缓冲区中取出一个字节
            decoded.push(byte); // 将字节添加到解码结果中
        }
    }
    
    decoded
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn decode() {
        let base64 = "QmFzZTY0REVDT0RFdGVzdA==";
        assert_eq!(decode_unchecked(base64), b"Base64DECODEtest");
    }
}