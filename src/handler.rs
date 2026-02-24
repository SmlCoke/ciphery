//! 命令分发与执行逻辑模块
//!
//! 本模块负责接收解析后的命令行参数，并调用对应的加密/解密引擎执行操作。
//! 将"做什么事"的逻辑与 CLI 参数定义和程序入口分离开来。

use ciphery::{Cipher, caesar, vigenere};
use dialoguer::{Input, Select, theme::ColorfulTheme};
use std::fs;
// ciphery代表外部的库Crate，使用具体的包名（如 ciphery、clap、std）代表引入一个外部的 Crate。
use crate::cli::{Algorithm, Commands};
// carte:: 代表当前 crate 的根模块，因为 handler.rs 是被 main.rs 声明和引入的模块，所以它属于你的二进制 Crate (Binary Crate)。在这里，crate:: 就等同于从 main.rs 开始查找。

// ====== 公共入口：根据子命令分发执行 ======
/// 根据解析到的子命令分发到对应的处理逻辑
pub fn run(command: Option<&Commands>) {
    match command {
        Some(Commands::Encrypt {
            text,
            algo,
            key,
            file_path,
        }) => {
            handle_encrypt(text, algo, key, file_path);
        }
        Some(Commands::Decrypt {
            text,
            algo,
            key,
            file_path,
        }) => {
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

    // 执行加密
    execute_encrypt(algorithm, &plaintext, key);
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

    // 执行解密
    execute_decrypt(algorithm, &ciphertext, key);
}

// ============================================================================
// 交互式 REPL 模式
// ============================================================================

/// 交互式 REPL 主循环
///
/// 用户直接运行 `ciphery`（不带子命令）时进入此模式。
/// 通过 `dialoguer` 库提供上下键选择的交互式菜单，循环执行直到用户选择退出。
fn handle_interactive() {
    println!("✨ Welcome to Ciphery's interactive mode!");
    println!("Type your choices below. Select 'Exit' to quit.\n");

    let theme = ColorfulTheme::default();

    loop {
        // ====== Step 1: 选择操作 ======
        let actions = &["Encrypt", "Decrypt", "Exit"];
        let action_index = match Select::with_theme(&theme)
            .with_prompt("What would you like to do?")
            .items(actions)
            .default(0)
            .interact()
        {
            Ok(idx) => idx,
            Err(_) => {
                println!("[error] Failed to read your selection. Exiting.");
                break;
            }
        };

        // 用户选择退出
        if action_index == 2 {
            break;
        }

        let is_encrypt = action_index == 0;

        // ====== Step 2: 选择算法 ======
        let algorithms = &["Caesar", "ROT13", "Vigenere", "Base64 (coming soon)"];
        let algo_index = match Select::with_theme(&theme)
            .with_prompt("Choose an algorithm")
            .items(algorithms)
            .default(0)
            .interact()
        {
            Ok(idx) => idx,
            Err(_) => {
                println!("[error] Failed to read your selection.");
                continue;
            }
        };

        let algorithm = match algo_index {
            0 => Algorithm::Caesar,
            1 => Algorithm::Rot13,
            2 => Algorithm::Vigenere,
            _ => {
                println!(
                    "[warning] This algorithm is not implemented yet. Please choose another.\n"
                );
                continue;
            }
        };

        // ====== Step 3: 选择文本来源：直接输入 or 文件传入 ======
        let text_source = &["Terminal", "File"];
        let text_source_index = match Select::with_theme(&theme)
            .with_prompt("Choose an algorithm")
            .items(text_source)
            .default(0)
            .interact()
        {
            Ok(idx) => idx,
            Err(_) => {
                println!("[error] Failed to read your selection.");
                continue;
            }
        };

        let text: String = match text_source_index {
            0 => match Input::with_theme(&theme)
                .with_prompt(if is_encrypt {
                    "Enter the text to encrypt"
                } else {
                    "Enter the text to decrypt"
                })
                .interact_text()
            {
                Ok(t) => t,
                Err(_) => {
                    println!("[error] Failed to read your input.");
                    continue;
                }
            },

            // 如果是文件，则从文件中读取文本
            _ => match Input::<String>::with_theme(&theme)
                .with_prompt(if is_encrypt {
                    "Enter the file path of text to encrypt"
                } else {
                    "Enter the file path of text to decrypt"
                })
                .interact_text()
            {
                Ok(fp) => {
                    // 去除用户可能误加的引号和空白
                    let cleaned_path = fp.trim().trim_matches('"').trim_matches('\'');
                    // 文件是否读取成功也需要模式匹配
                    match fs::read_to_string(cleaned_path) {
                        Ok(content) => content,
                        Err(e) => {
                            println!("[error] Failed to read file '{}': {}", cleaned_path, e);
                            continue;
                        }
                    }
                }
                Err(_) => {
                    println!("[error] Failed to read your input.");
                    continue;
                }
            },
        };

        // ====== Step 4: 输入密钥（如果算法需要） ======
        let key: Option<String> = match algorithm {
            Algorithm::Caesar | Algorithm::Vigenere => {
                let k: String = match Input::with_theme(&theme)
                    .with_prompt("Enter the key (e.g. shift amount, or keyword)")
                    .interact_text()
                {
                    Ok(k) => k,
                    Err(_) => {
                        println!("[error] Failed to read your input.");
                        continue;
                    }
                };
                Some(k)
            }
            // ROT13 等不需要密钥的算法
            _ => None,
        };

        // ====== Step 5: 执行加密/解密 ======
        println!(); // 空行，让输出更美观
        if is_encrypt {
            execute_encrypt(algorithm, &text, &key);
        } else {
            execute_decrypt(algorithm, &text, &key);
        }
        println!(); // 空行分隔，准备下一轮循环
    }
}

// ============================================================================
// 核心执行函数（供 CLI 模式和交互模式共用）
// ============================================================================

/// 执行加密操作
fn execute_encrypt(algorithm: Algorithm, text: &str, key: &Option<String>) {
    match algorithm {
        Algorithm::Caesar => {
            let shift = parse_caesar_key(key);
            let cipher = caesar::Caesar::new(shift);
            match cipher.encrypt(text) {
                Ok(encrypted) => println!("[result] Encrypted text:\n{}", encrypted),
                Err(e) => println!("[error] Encryption failed:\n{}", e),
            }
        }
        Algorithm::Rot13 => {
            let shift = 13;
            let cipher = caesar::Caesar::new(shift);
            match cipher.encrypt(text) {
                Ok(encrypted) => println!("[result] Encrypted text:\n{}", encrypted),
                Err(e) => println!("[error] Encryption failed:\n{}", e),
            }
        }
        Algorithm::Vigenere => {
            let key = key.as_ref().unwrap();
            let cipher = vigenere::Vigenere::new(key);
            match cipher.encrypt(text) {
                Ok(encrypted) => println!("[result] Encrypted text:\n{}", encrypted),
                Err(e) => println!("[error] Encryption failed:\n{}", e),
            }
        }
        _ => {
            println!("[error] Algorithm not implemented yet!");
        }
    }
}

/// 执行解密操作
fn execute_decrypt(algorithm: Algorithm, text: &str, key: &Option<String>) {
    match algorithm {
        Algorithm::Caesar => {
            let shift = parse_caesar_key(key);
            let cipher = caesar::Caesar::new(shift);
            match cipher.decrypt(text) {
                Ok(decrypted) => println!("[result] Decrypted text:\n{}", decrypted),
                Err(e) => println!("[error] Decryption failed:\n{}", e),
            }
        }
        Algorithm::Rot13 => {
            let shift = 13;
            let cipher = caesar::Caesar::new(shift);
            match cipher.decrypt(text) {
                Ok(decrypted) => println!("[result] Decrypted text:\n{}", decrypted),
                Err(e) => println!("[error] Decryption failed:\n{}", e),
            }
        }
        Algorithm::Vigenere => {
            let key = key.as_ref().unwrap();
            let cipher = vigenere::Vigenere::new(key);
            match cipher.encrypt(text) {
                Ok(encrypted) => println!("[result] Encrypted text:\n{}", encrypted),
                Err(e) => println!("[error] Encryption failed:\n{}", e),
            }
        }
        _ => {
            println!("[error] Algorithm not implemented yet!");
        }
    }
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
            Algorithm::Vigenere => {
                println!("[error] No key provided for Vigenere cipher!");
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
        .parse() // &String => Result<u8, ParseIntError>
        .expect("Key for Caesar cipher must be a number!");
    shift % 26
}

/// 程序结束时打印信息
fn print_exit_message() {
    println!("[info] Thanks for using Ciphery! Goodbye! 👋\n");
}
