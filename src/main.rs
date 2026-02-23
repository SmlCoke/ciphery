use clap::{Parser, Subcommand, ValueEnum};
use ciphery::{caesar, Cipher};
use std::fs;
// ============================================================================
// CLI 元数据定义 (方便后续统一修改和扩展)
// ============================================================================
const CLI_ABOUT: &str = "A lightweight interactive command-line encryption/decryption tool developed in Rust.";
const CLI_LONG_ABOUT: &str = "\
An interactive command-line encryption/decryption tool based on Rust, supporting multiple cryptographic algorithms.  
Maintained by SmlCoke(j.feng.st05@gmail.com).  

Source repository link: https://github.com/SmlCoke/ciphery 
Online demo: http://smlcoke.com
";

const CLI_AFTER_HELP: &str = "\
EXAMPLES:
    # Encrypt the text 'hello' using Caesar cipher with a shift of 3
    ciphery encrypt -t hello -a caesar -k 3

    # Decrypt the text 'khoor' using Caesar cipher with a shift of 3
    ciphery decrypt -t khoor -a caesar -k 3

    # Launch without any parameters to enter interactive REPL mode (to be implemented)
    ciphery
";

/// Ciphery - 你的交互式命令行加密解密工具
// `#[command(...)]` 会自动从你的 Cargo.toml 读取名字、版本和作者信息
#[derive(Parser, Debug)]
#[command(name = "ciphery")]
#[command(author = "SmlCoke <https://github.com/SmlCoke, http://smlcoke.com>")]
#[command(version)] // 自动从 Cargo.toml 读取版本号
#[command(about = CLI_ABOUT, long_about = CLI_LONG_ABOUT)]
#[command(after_help = CLI_AFTER_HELP)]
struct Cli {
    // 使用 Option 包裹子命令。
    // 核心逻辑：如果用户输入了子命令，值为 Some；如果只输入了 `ciphery`，值为 None。
    #[command(subcommand)]
    pub command: Option<Commands>,
}

// 定义子命令枚举：Encrypt 和 Decrypt
#[derive(Subcommand, Debug)]
enum Commands {
    /// Perform encryption operation
    Encrypt {
        /// 要加密的文本 (使用 -t 或 --text)
        #[arg(short, long)]
        text: Option<String>,
        // 这里用 Option 的原因是，我们允许用户不直接输入待加密/解密文本，而是传入文件路径

        /// 指定加密算法
        #[arg(short, long, value_enum, default_value_t = Algorithm::Caesar)]
        algo: Algorithm,

        /// 加密密钥 (对于凯撒密码，这是一个数字)
        #[arg(short, long)]
        key: Option<String>,
        // 这里用 Option 的原因是，不同算法可能需要不同类型的密钥，甚至有些算法不需要密钥（如 ROT13）。我们可以在后续逻辑中根据算法类型来验证和处理密钥。

        /// 待加密文本路径 
        #[arg(short, long)]
        file_path: Option<String>,
    },

    /// Perform decryption operation
    Decrypt {
        /// 要解密的文本
        #[arg(short, long)]
        text: Option<String>,

        /// 指定解密算法
        #[arg(short, long, value_enum, default_value_t = Algorithm::Caesar)]
        algo: Algorithm,

        /// 解密密钥
        #[arg(short, long)]
        key: Option<String>,

        /// 待解密文本路径 
        #[arg(short, long)]
        file_path: Option<String>,
    },
}

// 极其推荐的做法：为算法提供一个 ValueEnum！
// 这样如果用户输入了错别字（比如 --algo ceasar），Clap 会直接报错并提示正确的拼写。
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Algorithm {
    Caesar,
    Rot13,
    Base64,
}

fn main() {
    // 这一行施展魔法：解析命令行参数并填充到 Cli 结构体中
    let cli = Cli::parse();

    // 根据解析结果进行模式匹配
    match &cli.command {
        // 匹配到了加密命令
        Some(Commands::Encrypt { text, algo, key, file_path }) => {
            println!("[info] Encryption mode...");
            println!("[info] Algorithm: {:?}", algo);
            
            let t = if let Some(t) = text {
                println!("[info] Text to be encrypted: {}", t);
                t.clone() // 这里需要 clone，因为 t 是 &String，而我们后续可能需要 String 类型
            } else {
                println!("[warning] No text provided, may need to read from file (to be implemented)");
                if let Some(fp) = file_path {
                    println!("[info] Reading text from file: {}", fp);
                    match fs::read_to_string(fp) {
                        Ok(content) => content,
                        Err(e) => {
                            println!("[error] Failed to read file: {}", e);
                            return;
                        }
                    }
                } else {
                    println!("[error] No text or file path provided!");
                    return; // 直接退出 main 函数
                }
            };

            let algorithm = *algo; // algo 已经是 Algorithm 类型，不需要 unwrap，直接解引用即可

            if let Some(k) = key {
                println!("[info] Key used: {}", k);
            } else  {
                if algorithm == Algorithm::Caesar {
                    println!("[error] No key provided for Caesar cipher!");
                    return; // 直接退出 main 函数
                }
            }

            // ====== 根据不同算法进行加密处理 ======
            match algorithm {
                // ======= Caesar 加密逻辑 =======
                Algorithm::Caesar => {
                    // 这里我们需要把 key 从 String 转换成 u8
                    let shift: u8 = key.as_ref().unwrap().parse().expect("Key for Caesar cipher must be a number!");
                    let shift = shift % 26; // 确保偏移量在 0-25 之间
                    let cipher = caesar::Caesar::new(shift);
                    // 注意：encrypt 返回的是 Result，我们需要处理它
                    let encrypted = match cipher.encrypt(&t) {
                        Ok(enc) => enc,
                        Err(e) => {
                            println!("[error] Encryption failed: {}", e);
                            return; // 直接退出 main 函数
                        }
                    };
                    println!("[info] Encrypted text\n {}", encrypted);
                },
                _ => {
                    println!("[error] Algorithm not implemented yet!");
                }
            }

        }
        // 匹配到了解密命令
        Some(Commands::Decrypt { text, algo, key, file_path }) => {
            println!("[info] Decryption mode...");
            println!("[info] Algorithm: {:?}", algo);
            

            let t = if let Some(t) = text {
                println!("[info] Text to be decrypted: {}", t);
                t.clone()
            } else {
                println!("[warning] No text provided, may need to read from file (to be implemented)");
                if let Some(fp) = file_path {
                    println!("[info] Reading text from file: {}", fp);
                    match fs::read_to_string(fp) {
                        Ok(content) => content,
                        Err(e) => {
                            println!("[error] Failed to read file: {}", e);
                            return;
                        }
                    }
                } else {
                    println!("[error] No text or file path provided!");
                    return; // 直接退出 main 函数
                }
            };

            let algorithm = *algo; // algo 已经是 Algorithm 类型，不需要 unwrap，直接解引用即可

            if let Some(k) = key {
                println!("[info] Key used: {}", k);
            } else  {
                if algorithm == Algorithm::Caesar {
                    println!("[error] No key provided for Caesar cipher!");
                    return; // 直接退出 main 函数
                }
            }
            
            // ====== 根据不同算法进行解密处理 ======
            match algorithm {
                // ======= Caesar 解密逻辑 =======
                Algorithm::Caesar => {
                    let shift: u8 = key.as_ref().unwrap().parse().expect("Key for Caesar cipher must be a number!");
                    let shift = shift % 26; // 确保偏移量在 0-25 之间
                    let cipher = caesar::Caesar::new(shift);
                    // 注意：decrypt 返回的是 Result，我们需要处理它
                    let decrypted = match cipher.decrypt(&t) {
                        Ok(dec) => dec,
                        Err(e) => {
                            println!("[error] Decryption failed: {}", e);
                            return; // 直接退出 main 函数
                        }
                    };
                    println!("[info] Decrypted text\n {}", decrypted);
                },
                _ => {
                    println!("[error] Algorithm not implemented yet!");
                }
            }
        }

        // 用户只输入了 `ciphery`，没有带任何参数
        None => {
            println!("✨ 欢迎来到 Ciphery 交互模式！");
            println!("(正在准备启动交互式菜单...)");
            // TODO: Step 5 的交互式 REPL 逻辑将在这里展开
        }
    }
}