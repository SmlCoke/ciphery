//! 命令分发与执行逻辑模块
//!
//! 本模块负责接收解析后的命令行参数，并调用对应的加密/解密引擎执行操作。
//! 将"做什么事"的逻辑与 CLI 参数定义和程序入口分离开来。

use std::fs;
use ciphery::{caesar, Cipher};
// ciphery代表外部的库Crate，使用具体的包名（如 ciphery、clap、std）代表引入一个外部的 Crate。
use crate::cli::{Algorithm, Commands};
// carte:: 代表当前 crate 的根模块，因为 handler.rs 是被 main.rs 声明和引入的模块，所以它属于你的二进制 Crate (Binary Crate)。在这里，crate:: 就等同于从 main.rs 开始查找。

// ====== 公共入口：根据子命令分发执行 ======
/// 根据解析到的子命令分发到对应的处理逻辑
pub fn run(command: Option<&Commands>) {
    match command {
        Some(Commands::Encrypt { text, algo, key, file_path }) => {
            handle_encrypt(text, algo, key, file_path);
        }
        Some(Commands::Decrypt { text, algo, key, file_path }) => {
            handle_decrypt(text, algo, key, file_path);
        }
        None => {
            handle_interactive();
        }
    }
    print_exit_message();
}

// ====== 子命令执行器 ======
/// 处理加密命令
fn handle_encrypt(
    text: &Option<String>,
    algo: &Algorithm,
    key: &Option<String>,
    file_path: &Option<String>,
) {
    println!("[info] Encryption mode...");
    println!("[info] Algorithm: {:?}", algo);

    // 获取待加密文本：优先使用 --text，其次从 --file-path 读取
    let plaintext = match resolve_input_text(text, file_path) {
        Some(t) => t,
        None => return,
    };

    let algorithm = *algo;

    // 校验密钥
    if !validate_key(key, algorithm) {
        return;
    }

    // 根据不同算法进行加密处理
    match algorithm {
        Algorithm::Caesar => {
            let shift = parse_caesar_key(key);
            let cipher = caesar::Caesar::new(shift);
            match cipher.encrypt(&plaintext) {
                Ok(encrypted) => println!("[info] Encrypted text:\n{}", encrypted),
                Err(e) => println!("[error] Encryption failed:\n{}", e),
            }
        }
        _ => {
            println!("[error] Algorithm not implemented yet!");
        }
    }
}

/// 处理解密命令
fn handle_decrypt(
    text: &Option<String>,
    algo: &Algorithm,
    key: &Option<String>,
    file_path: &Option<String>,
) {
    println!("[info] Decryption mode...");
    println!("[info] Algorithm: {:?}", algo);

    // 获取待解密文本
    let ciphertext = match resolve_input_text(text, file_path) {
        Some(t) => t,
        None => return,
    };

    let algorithm = *algo;

    // 校验密钥
    if !validate_key(key, algorithm) {
        return;
    }

    // 根据不同算法进行解密处理
    match algorithm {
        Algorithm::Caesar => {
            let shift = parse_caesar_key(key);
            let cipher = caesar::Caesar::new(shift);
            match cipher.decrypt(&ciphertext) {
                Ok(decrypted) => println!("[info] Decrypted text:\n{}", decrypted),
                Err(e) => println!("[error] Decryption failed:\n{}", e),
            }
        }
        _ => {
            println!("[error] Algorithm not implemented yet!");
        }
    }
}

/// 处理交互式模式（暂未实现）
fn handle_interactive() {
    println!("✨ 欢迎来到 Ciphery 交互模式！");
    println!("(正在准备启动交互式菜单...)");
    // TODO: Step 5 的交互式 REPL 逻辑将在这里展开
}


// ====== 辅助工具函数 ======
/// 解析输入文本：优先使用命令行直接输入的 text，其次从文件路径读取
fn resolve_input_text(text: &Option<String>, file_path: &Option<String>) -> Option<String> {
    if let Some(t) = text {
        println!("[info] Input text: {}", t);
        Some(t.clone())
    } else if let Some(fp) = file_path {
        println!("[info] Reading text from file: {}", fp);
        match fs::read_to_string(fp) {
            Ok(content) => Some(content),
            Err(e) => {
                println!("[error] Failed to read file: {}", e);
                None
            }
        }
    } else {
        println!("[error] No text or file path provided!");
        None
    }
}

/// 校验密钥是否已提供（对于需要密钥的算法）
// 注意，这里的 key 还是从 CLI 解析到的，其类型当然还是 &Option<String>
fn validate_key(key: &Option<String>, algorithm: Algorithm) -> bool {
    if let Some(k) = key {
        println!("[info] Key used: {}", k);
        true
    } else {
        // 根据算法判断是否必须提供密钥
        match algorithm {
            Algorithm::Caesar => {
                println!("[error] No key provided for Caesar cipher!");
                false
            }
            // ROT13 / Base64 等不需要密钥的算法可以在这里放行
            _ => true,
        }
    }
}

/// 解析凯撒密码的密钥（从 String 转为 u8 偏移量）
fn parse_caesar_key(key: &Option<String>) -> u8 {
    let shift: u8 = key
        .as_ref() // 从 &Option<String> => Option<&String>
        .unwrap() // Option<&String> => &String
        .parse()  // &String => Result<u8, ParseIntError>
        .expect("Key for Caesar cipher must be a number!");
    shift % 26
}

/// 程序结束时打印字段
fn print_exit_message() {
    println!("[info] Thanks for your using Ciphery! Goodbye! 👋\n");
}