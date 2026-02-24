//! 凯撒密码 (Caesar Cipher) 的实现
//!
//! 凯撒密码是一种简单的替换加密技术，通过将字母表中的每个字母移动固定数量的位置来进行加密。

/// 凯撒密码加密函数
///
/// # 参数
///
/// * `text` - 需要加密的明文字符串切片 (`&str`)
/// * `shift` - 偏移量 (`u8`)，实际有效偏移量为 `shift % 26`
///
/// # 返回值
///
/// 返回加密后的 `String`
///
/// # 示例
///
/// ```
/// use ciphery::caesar::encrypt;
/// let encrypted = encrypt("hello", 3);
/// assert_eq!(encrypted, "khoor");
/// ```
pub fn encrypt(text: &str, shift: u8) -> String {
    // 确保偏移量在 0-25 之间，避免溢出
    let shift = shift % 26;

    // 使用迭代器和闭包处理字符串，这是 Rust 中处理集合的惯用且高效的方式
    // 这里我们选择用 .chars() 获取迭代器，它将字节流解析为一个个独立的 Unicode 字符（`char` 类型，每个 `char` 固定占 4 字节）。
    text.chars()
        .map(|c| {
            // 使用模式匹配处理不同类型的字符
            match c {
                // 处理小写字母
                'a'..='z' => {
                    let offset = c as u8 - b'a';
                    let new_offset = (offset + shift) % 26;
                    (b'a' + new_offset) as char
                }
                // 处理大写字母
                'A'..='Z' => {
                    let offset = c as u8 - b'A';
                    let new_offset = (offset + shift) % 26;
                    (b'A' + new_offset) as char
                }
                // 非字母字符（如数字、标点符号、空格、中文）保持不变
                _ => c,
            }
        })
        .collect() // 将迭代器收集为 String
}

/// 凯撒密码解密函数
///
/// # 参数
///
/// * `text` - 需要解密的密文字符串切片 (`&str`)
/// * `shift` - 加密时使用的偏移量 (`u8`)
///
/// # 返回值
///
/// 返回解密后的 `String`
///
/// # 示例
///
/// ```
/// use ciphery::caesar::decrypt;
/// let decrypted = decrypt("khoor", 3);
/// assert_eq!(decrypted, "hello");
/// ```
pub fn decrypt(text: &str, shift: u8) -> String {
    // 解密相当于反向偏移，即加上 26 - (shift % 26)
    let shift = shift % 26;
    let reverse_shift = if shift == 0 { 0 } else { 26 - shift };

    // 复用 encrypt 函数，体现代码复用原则
    encrypt(text, reverse_shift)
}

// 引入核心 Trait 和错误类型
use crate::{Cipher, CipherError};

/// 凯撒密码结构体
///
/// 这是一个持有状态（偏移量）的结构体，它实现了 `Cipher` Trait。
/// 这种设计模式允许我们将不同的加密算法统一抽象为 `Cipher` 对象。
pub struct Caesar {
    /// 凯撒密码的偏移量
    shift: u8,
}

impl Caesar {
    /// 创建一个新的凯撒密码实例
    ///
    /// # 参数
    ///
    /// * `shift` - 偏移量，会自动对 26 取模
    pub fn new(shift: u8) -> Self {
        Self { shift: shift % 26 }
    }
}

// 为 Caesar 结构体实现 Cipher Trait
// 这是 Rust 中实现多态和接口抽象的核心机制
impl Cipher for Caesar {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        // 凯撒密码的加密过程不会失败，因此我们直接调用底层函数并用 Ok 包装
        // 在更复杂的算法（如 AES）中，这里可能会返回 Err(CipherError::InvalidInput(...))
        Ok(encrypt(text, self.shift))
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        // 同理，解密过程也不会失败
        Ok(decrypt(text, self.shift))
    }
}

// 单元测试模块
#[cfg(test)]
mod tests {
    // 引入父模块中的所有公共项
    use super::*;

    #[test]
    fn test_encrypt_lowercase() {
        assert_eq!(encrypt("hello", 3), "khoor");
    }

    #[test]
    fn test_encrypt_uppercase() {
        assert_eq!(encrypt("HELLO", 3), "KHOOR");
    }

    #[test]
    fn test_encrypt_mixed_and_punctuation() {
        assert_eq!(encrypt("Hello, World!", 5), "Mjqqt, Btwqi!");
    }

    #[test]
    fn test_encrypt_large_shift() {
        // 29 % 26 == 3
        assert_eq!(encrypt("hello", 29), "khoor");
    }

    #[test]
    fn test_decrypt_lowercase() {
        assert_eq!(decrypt("khoor", 3), "hello");
    }

    #[test]
    fn test_decrypt_mixed_and_punctuation() {
        assert_eq!(decrypt("Mjqqt, Btwqi!", 5), "Hello, World!");
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        // 测试加密和解密是否能完美还原
        let original = "Rust is awesome! 123";
        let shift = 17;

        let encrypted = encrypt(original, shift);
        // encrypted 类型是 String，因此传入 decrypt 时需要转换为 &str
        let decrypted = decrypt(&encrypted, shift);

        assert_eq!(original, decrypted);
    }

    #[test]
    fn test_caesar_struct_and_trait() {
        // 测试面向对象风格的 Trait 抽象
        let cipher = Caesar::new(3);

        // 调用 Trait 方法，返回的是 Result，所以需要 unwrap()
        let encrypted = cipher.encrypt("hello").unwrap();
        assert_eq!(encrypted, "khoor");

        let decrypted = cipher.decrypt(&encrypted).unwrap();
        assert_eq!(decrypted, "hello");
    }

    #[test]
    fn test_rot13() {
        // Rot13 是 shift = 13 的 caesar 算法，满足加密两次后还原（因为13*2%26=0）
        // 因此 Rot13 算法不再单独创建 .rs 文件，由本模块完成测试
        let cipher = Caesar::new(13);

        // 调用 Trait 方法，返回的是 Result，所以需要 unwrap()
        let encrypted = cipher.encrypt("hello").unwrap();
        assert_eq!(encrypted, "uryyb");

        let decrypted = cipher.encrypt(&encrypted).unwrap();
        assert_eq!(decrypted, "hello");
    }
}
