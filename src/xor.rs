// 引入核心 Trait 和错误类型
use crate::{Cipher, CipherError};

pub struct Xor {
  key: Vec<u8>,
}

impl Xor {
    /// 创建一个新的 Xor 密码实例
    ///
    /// # 参数
    ///
    /// * `key` - 密钥
    pub fn new(key: &str) -> Self {
        if key.is_empty() {
            panic!("Key cannot be empty");
        }
        Xor { key: key.bytes().collect() }
        // key.bytes() 返回一个迭代器，我们使用 collect() 将其转换为 Vec<u8>，方便后续按索引访问
    }
}

impl Cipher for Xor {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let xored_bytes: Vec<u8> = text.bytes()
            .zip(self.key.iter().cycle())
            .map(|(text_byte, key_byte)| {text_byte^key_byte})
            .collect();

        // hex::encode 返回类型是 String
        Ok(hex::encode(xored_bytes))
        
    }

    fn decrypt(&self, hex_text: &str) -> Result<String, CipherError> {
        let text_bytes = hex::decode(hex_text)
            .map_err(|e| CipherError::HexCodingError(format!("XOR decryption failed: {}", e)))?;
        // hex::decode 可能失败，所以我们使用 map_err 将错误转换为 CipherError::HexCodingError
        // hex::decode 返回类型是 Result<Vec<u8>, hex::FromHexError>，我们需要处理这个错误

        let decrypted_bytes: Vec<u8> = text_bytes.iter()
            .zip(self.key.iter().cycle())
            .map(|(text_byte, key_byte)| text_byte ^ key_byte)
            .collect();

        match String::from_utf8(decrypted_bytes) {
            Ok(res) => Ok(res),
            Err(e) => Err(CipherError::HexCodingError(format!("XOR decryption failed: {}", e))),
        }
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor_encrypt_decrypt() {
        let text = "Hello 🦀 (Rust) 世界!"; // 包含英文、Emoji、符号、中文
        let key = "super_secret_key_123";
        let cipher = Xor::new(key);
        // 1. 测试加密
        let encrypted_hex = cipher.encrypt(text).unwrap();
        println!("加密后的 Hex: {}", encrypted_hex);
        
        // 确保加密后长得完全不一样
        assert_ne!(text, encrypted_hex); 

        // 2. 测试解密
        let decrypted_text = cipher.decrypt(&encrypted_hex).unwrap();
        assert_eq!(decrypted_text, text);
    }

    #[test]
    fn test_xor_invalid_hex_decrypt() {
        // 测试用户瞎输解密内容的情况
        let invalid_hex = "this is not hex";
        let result = Xor::new("key").decrypt(invalid_hex);
        assert!(result.is_err());
    }
}