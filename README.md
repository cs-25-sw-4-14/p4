# Lossy programming language

## Setting up LLVM (MacOS)
1. Install LLVM using Homebrew:
   ```bash
   brew install llvm@18
   ```

2. Add LLVM to your PATH:
   ```bash
    export LLVM_SYS_180_PREFIX="/opt/homebrew/opt/llvm@18"
    ```
    Add this line to your `~/.zshrc` or `~/.bash_profile` to make it permanent.
