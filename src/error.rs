//! 错误处理模块
//!
//! 定义了 `ciphery` 库中可能出现的各种错误类型。

use std::fmt;

/// 加密/解密过程中可能发生的错误
///
/// 这是一个自定义的枚举类型，用于表示加密引擎中可能出现的各种异常情况。
/// 派生了 `Debug` 和 `PartialEq`，方便在测试中进行断言比较。
#[derive(Debug, PartialEq)]
pub enum CipherError {
    /// 输入数据无效（例如：包含了算法不支持的字符）
    InvalidInput(String),
    /// 密钥无效（例如：密钥长度不符合要求）
    InvalidKey(String),
    /// 十六进制编码错误
    HexCodingError(String),
    /// 其他内部错误
    Other(String),
}

// 实现 Display Trait，用于面向用户的友好错误打印
// 这是 Rust 错误处理的标准做法，使得错误信息可以被格式化输出（如 `println!("{}", err)`）
impl fmt::Display for CipherError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CipherError::InvalidInput(msg) => write!(f, "无效的输入: {}", msg),
            CipherError::InvalidKey(msg) => write!(f, "无效的密钥: {}", msg),
            CipherError::HexCodingError(msg) => write!(f, "十六进制编码错误: {}", msg),
            CipherError::Other(msg) => write!(f, "加密引擎内部错误: {}", msg),
        }
    }
}

// 实现 Error Trait，使其成为标准的 Rust 错误类型
// 这样我们的错误类型就可以与 `Box<dyn std::error::Error>` 兼容，
// 并且可以无缝使用 `?` 运算符进行错误传播。
impl std::error::Error for CipherError {}
