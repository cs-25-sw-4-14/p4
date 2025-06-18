# Lossy Programming Language

**Lossy** is an experimental programming language focused on minimalism and compile-time safety. It's built using LLVM to take advantage of powerful compiler infrastructure.

> ⚠️ **Note**: Lossy is currently under active development. Breaking changes may occur.

---

## 🚀 Features

* Simple, expressive syntax
* LLVM-based compilation
* Memory-safe by design

---

## 💠 Installation

### Prerequisites
* Clang
* LLVM 18

### Install LLVM (macOS)
1. Install via Homebrew:

   ```bash
   brew install llvm@18
   ```

2. Configure environment variables:

   ```bash
   export LLVM_SYS_180_PREFIX="/opt/homebrew/opt/llvm@18"
   ```

   To persist this setting, add the line above to your shell config file (e.g., `~/.zshrc` or `~/.bash_profile`), then restart your terminal or run:

   ```bash
   source ~/.zshrc   # or ~/.bash_profile
   ```

---

## 🧪 Building Lossy

Clone the repository and build the project:

```bash
git clone https://github.com/cs-25-sw-4-14/p4
cd lossy
cargo build --release
```

---

## ✍️ Writing and Running Lossy Code

Here's an example of a simple Lossy program:

```lossy
void main() {
    debug("Hello, world!\n");
}
```

To compile and run it:

```bash
./target/release/lossy hello.ss
./hello.out
```

---

## 📄 License

This project is licensed under the MIT License – see the [LICENSE](LICENSE) file for details.

---
