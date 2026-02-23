//! `ciphery` 核心加密引擎库
//!
//! 该库提供了多种加密算法的实现，目前包含：
//! - 凯撒密码 (Caesar Cipher)

pub mod caesar;
pub mod error;

// 重新导出（Re-export），方便外部直接使用 `ciphery::CipherError` 和 `ciphery::Cipher`
pub use error::CipherError;

/// 核心加密 Trait，定义了所有加密算法的共享行为
///
/// 任何实现了此 Trait 的结构体都可以被视为一种加密算法，
/// 这为后续实现多态和动态分发（如 `Box<dyn Cipher>`）打下了基础。
pub trait Cipher {
    /// 加密给定的明文
    ///
    /// 成功时返回 `Ok(String)`，失败时返回 `Err(CipherError)`
    fn encrypt(&self, text: &str) -> Result<String, CipherError>;

    /// 解密给定的密文
    ///
    /// 成功时返回 `Ok(String)`，失败时返回 `Err(CipherError)`
    fn decrypt(&self, text: &str) -> Result<String, CipherError>;
}
