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


// ==========================================
// WebAssembly (WASM) 暴露接口
// ==========================================
#[cfg(target_arch = "wasm32")] // 只在编译目标为 wasm32 时生效
#[cfg(feature = "wasm")] // 只在启用 wasm feature 时生效
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn wasm_encrypt(algo: &str, text: &str, key: &str) -> String {
    match algo {
        "caesar" => {
            // 解析密钥
            let shift: u8 = key.parse().unwrap_or(0) % 26;
            let cipher = crate::caesar::Caesar::new(shift);
            
            // 调用你原有的 encrypt 方法（根据你的代码结构，这里假设返回 Result<String, _>）
            match cipher.encrypt(text) {
                Ok(res) => res,
                Err(e) => format!("Error: {}", e),
            }
        },
        _ => format!("Algorithm '{}' not supported yet in Web", algo),
    }
}

#[cfg(target_arch = "wasm32")]
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn wasm_decrypt(algo: &str, text: &str, key: &str) -> String {
    match algo {
        "caesar" => {
            let shift: u8 = key.parse().unwrap_or(0) % 26;
            let cipher = crate::caesar::Caesar::new(shift);
            
            match cipher.decrypt(text) {
                Ok(res) => res,
                Err(e) => format!("Error: {}", e),
            }
        },
        _ => format!("Algorithm '{}' not supported yet in Web", algo),
    }
}