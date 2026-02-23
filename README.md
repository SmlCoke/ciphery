# Ciphery 🔐

[English](README.md) | [中文](README_zh.md)

A lightweight interactive command-line encryption/decryption tool developed in Rust.

**Ciphery** is a lightweight, interactive command-line text encryption/decryption tool built with Rust. It supports both quick single-line command execution like traditional CLI tools and a user-friendly interactive REPL menu mode.

## ✨ Features

- **Interactive Mode**: Simply run `ciphery` to enter an interactive menu with up/down arrow key selection, eliminating the need to memorize complex command-line arguments.
- **CLI Mode**: Supports quick task execution via subcommands (`encrypt` / `decrypt`), perfect for script automation.
- **Multiple Data Sources**: Supports direct text input in the terminal, as well as reading long texts from file paths for encryption/decryption.
- **Multiple Algorithms**:
  - ✅ Caesar Cipher
  - 🚧 ROT13 (Coming soon)
  - 🚧 Base64 (Coming soon)

## 🚀 Installation

Currently, you can install it directly by compiling from the source code. Please ensure you have the [Rust environment (Cargo)](https://www.rust-lang.org/tools/install) installed on your computer.

```bash
# 1. Clone the repository
git clone https://github.com/SmlCoke/ciphery.git
cd ciphery

# 2. Compile and install to the local Cargo bin directory
cargo install --path .
```
*Once installed, you can use the `ciphery` command anywhere in your terminal.*

## 💡 Usage

### 1. Interactive Mode (Recommended)
Simply type `ciphery` in your terminal and follow the on-screen prompts to make selections and input data:
```bash
ciphery
```

### 2. CLI Mode
If you know the exact parameters you need, you can use subcommands directly:

**Encrypt Text:**
```bash
# Encrypt the string "hello" using Caesar cipher (shift of 3)
ciphery encrypt -t "hello" -a caesar -k 3
```

**Decrypt Text:**
```bash
# Decrypt the string "khoor" using Caesar cipher (shift of 3)
ciphery decrypt -t "khoor" -a caesar -k 3
```

**Encrypt/Decrypt using a file:**
```bash
# Encrypt the contents of the input.txt file
ciphery encrypt --file-path input.txt -a caesar -k 3
```

### View Help
```bash
ciphery --help
ciphery encrypt --help
```

## 📸 Screenshots


**Interactive Menu Display:**
![Interactive Mode Placeholder](assets/png/interactive.png)

**CLI Execution Display:**
![CLI Mode Placeholder](assets/png/CLI.png)

---
**Author:** [SmlCoke](https://github.com/SmlCoke) | **Online Demo:** [smlcoke.com](http://smlcoke.com)
