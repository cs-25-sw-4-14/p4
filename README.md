# The Lossy Programming Language 🌀

Lossy is a high-performance, memory-efficient programming language designed to automatically truncate less important memory to avoid OOMKill (Out-Of-Memory Kill) scenarios. It features a borrow checker for memory safety and ensures fast and safe execution of programs.

## Features

- **Automatic Memory Truncation**: Lossy automatically truncates less important memory to prevent OOMKill scenarios, ensuring your programs run smoothly even under heavy memory usage.
- **Borrow Checker**: Ensures memory safety by preventing data races and dangling pointers.
- **High Performance**: Optimized for speed, making it suitable for performance-critical applications.
- **Memory Safety**: Guarantees safe memory access, reducing the risk of memory-related bugs.

## Installation

To install the Lossy programming language, follow these steps:

1. Clone the repository:
    ```sh
    git clone https://github.com/yourusername/lossy.git

2. Generate ANTLR files:
    ```sh
    ./antlr.sh

3. Compile compiler:
    ```sh
    ./build.sh

## Contributing

We welcome contributions to the Lossy programming language! If you would like to contribute, please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bugfix.
3. Commit your changes and push the branch to your fork.
4. Create a pull request with a description of your changes.

## License

Lossy is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.

