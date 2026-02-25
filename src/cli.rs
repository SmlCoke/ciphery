//! 命令行参数定义模块
//!
//! 本模块使用 `clap` 的 derive API 定义了所有的命令行参数结构、
//! 子命令以及算法枚举，将 CLI 的"长什么样"与"做什么事"分离开来。

use clap::{Parser, Subcommand, ValueEnum};

// ============================================================================
// CLI 元数据定义 (方便后续统一修改和扩展)
// ============================================================================
const CLI_ABOUT: &str =
    "A lightweight interactive command-line encryption/decryption tool developed in Rust.";

// 可视化横幅：clap 的 --help 长描述
const CLI_LONG_ABOUT: &str = concat!(
    "\n",
    "══════════════════════════════════════════════════════════\n",
    "  ✦  C I P H E R Y    ·    v", env!("CARGO_PKG_VERSION"), "\n",
    "  A Lightweight Command-Line Encryption / Decryption Tool\n",
    "══════════════════════════════════════════════════════════\n",
    "  Author  :  SmlCoke <j.feng.st05@gmail.com>\n",
    "  Repo    :  https://github.com/SmlCoke/ciphery\n",
    "  Demo    :  http://smlcoke.com\n",
    "══════════════════════════════════════════════════════════\n",
    "\n",
    "Supports multiple algorithms: Caesar, ROT13, Vigenere, XOR, Base64.\n",
    "Run without arguments to enter the interactive REPL mode.\n",
);

/// 在交互式模式（以及任何希望打印横幅的地方）调用此函数。
pub fn print_banner() {
    println!(
        "{}",
        concat!(
            "\n",
            "══════════════════════════════════════════════════════════\n",
            "  ✦  C I P H E R Y    ·    v", env!("CARGO_PKG_VERSION"), "\n",
            "  A Lightweight Command-Line Encryption / Decryption Tool\n",
            "══════════════════════════════════════════════════════════\n",
            "  Author  :  SmlCoke <j.feng.st05@gmail.com>\n",
            "  Repo    :  https://github.com/SmlCoke/ciphery\n",
            "  Demo    :  http://smlcoke.com\n",
            "══════════════════════════════════════════════════════════",
        )
    );
}

const CLI_AFTER_HELP: &str = "\
EXAMPLES:
    # Encrypt the text 'hello' using Caesar cipher with a shift of 3
    ciphery encrypt -t hello -a caesar -k 3

    # Decrypt the text 'khoor' using Caesar cipher with a shift of 3
    ciphery decrypt -t khoor -a caesar -k 3

    # Launch without any parameters to enter interactive REPL mode (to be implemented)
    ciphery
";

// ============================================================================
// CLI 参数结构体定义
// ============================================================================

/// Ciphery - 你的交互式命令行加密解密工具
#[derive(Parser, Debug)]
#[command(name = "ciphery")]
#[command(author = "SmlCoke <https://github.com/SmlCoke, http://smlcoke.com>")]
#[command(version)]
#[command(about = CLI_ABOUT, long_about = CLI_LONG_ABOUT)]
#[command(after_help = CLI_AFTER_HELP)]
pub struct Cli {
    /// 使用 Option 包裹子命令。
    /// 核心逻辑：如果用户输入了子命令，值为 Some；如果只输入了 `ciphery`，值为 None。
    #[command(subcommand)]
    pub command: Option<Commands>,
}

// ============================================================================
// 子命令定义
// ============================================================================

/// 定义子命令枚举：Encrypt 和 Decrypt
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Perform encryption operation
    Encrypt {
        /// 要加密的文本 (使用 -t 或 --text)
        #[arg(short, long)]
        text: Option<String>,

        /// 指定加密算法
        #[arg(short, long, value_enum, default_value_t = Algorithm::Caesar)]
        algo: Algorithm,

        /// 加密密钥 (对于凯撒密码，这是一个数字)
        #[arg(short, long)]
        key: Option<String>,

        /// 待加密文本的文件路径
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

        /// 待解密文本的文件路径
        #[arg(short, long)]
        file_path: Option<String>,
    },
}

// ============================================================================
// 算法枚举定义
// ============================================================================

/// 支持的加密算法
///
/// 使用 `ValueEnum` 宏使其可以直接作为命令行参数的值被解析。
/// 如果用户输入了不存在的算法名（如 `--algo ceasar`），Clap 会自动报错并提示正确拼写。
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Algorithm {
    Caesar,
    Rot13,
    Base64,
    Vigenere,
    Xor,
}
