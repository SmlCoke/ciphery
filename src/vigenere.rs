// 引入核心 Trait 和错误类型
use crate::{Cipher, CipherError};

pub struct Vigenere {
    key: Vec<u8>, // 存储密钥的字节数组，方便按索引访问
    len: usize,
}

impl Vigenere {
    /// 创建一个新的 Vigenere 密码实例
    ///
    /// # 参数
    ///
    /// * `key` - 密钥
    pub fn new(key: &str) -> Self {
        // 如果密钥为空，直接退出
        if key.is_empty() {
            panic!("Key cannot be empty");
        }

        // 如果含有非英文字母，直接退出
        if !key.chars().all(|c| c.is_ascii_alphabetic()) {
            panic!("Key must contain only ASCII letters");
        }

        let key = key.to_uppercase(); // 将密钥转换为大写，简化加密逻辑
        let len = key.len();
        let key_bytes = key.into_bytes(); // 转移所有权，避免悬空引用

        Self { key: key_bytes, len }
    }
}

impl Cipher for Vigenere {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut key_index = 0;
        Ok(text.chars().map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
                let key_char = self.key[key_index % self.len];
                let shift = (key_char - b'A') as u8; // 计算当前密钥字符的偏移量
                key_index += 1; // 只有当遇到字母时才增加密钥索引
                ((c as u8 - base + shift) % 26 + base) as char
            } else {
                c // 非字母字符直接返回，不加密
            }
        })
        .collect::<String>()
        )
        // Vigenre 算法不会出错，直接 Ok 返回加密结果
        
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut key_index = 0;
        Ok(text.chars().map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
                let key_char = self.key[key_index % self.len];
                let shift = (key_char - b'A') as u8; // 计算当前密钥字符的偏移量
                key_index += 1; // 只有当遇到字母时才增加密钥索引
                ((c as u8 - base + 26 - shift) % 26 + base) as char
            } else {
                c // 非字母字符直接返回，不加密
            }
        })
        .collect::<String>()
        )
        // Vigenre 算法不会出错，直接 Ok 返回加密结果
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vigenere_encrypt() {
        let text = "ATTACK AT DAWN!";
        let key = "LEMON";
        let cipher = Vigenere::new(key);
        let encrypted = cipher.encrypt(text).unwrap();
        assert_eq!(encrypted, "LXFOPV EF RNHR!"); // 注意：空格和感叹号完美保留
    }

    #[test]
    fn test_vigenere_decrypt() {
        let text = "LXFOPV EF RNHR!";
        let key = "LEMON";
        let cipher = Vigenere::new(key);
        let decrypted = cipher.decrypt(text).unwrap();
        assert_eq!(decrypted, "ATTACK AT DAWN!");
    }

    #[test]
    fn test_vigenere_with_unicode() {
        let text = "Hello 世界";
        let key = "KEY";
        let cipher = Vigenere::new(key);
        let encrypted = cipher.encrypt(text).unwrap();
        assert_eq!(encrypted, "Rijvs 世界");
        assert_eq!(cipher.decrypt(&encrypted).unwrap(), text);
    }
}