# Rust CLI Calculator ![GitHub forks](https://img.shields.io/github/forks/JCSUCoder/rust-cli-calculator?style=social) ![GitHub stars](https://img.shields.io/github/stars/JCSUCoder/rust-cli-calculator?style=social) ![GitHub watchers](https://img.shields.io/github/watchers/JCSUCoder/rust-cli-calculator?style=social) ![GitHub](https://img.shields.io/github/license/JCSUCoder/rust-cli-calculator)
This a simple command line interface calculator that can take complex expresions with parentheses and give a result.

**"^", "(", ")" still unsupported**

## To Build

```bash
git clone https://github.com/JCSUCoder/rust-cli-calculator
cd rust-cli-calculator
cargo build --release
```

## To Use

```bash
cd target/release/
./cli-calc 
```

```
Welcome to this CLI Rust Calculator
Write down the expression to solve or type 0 to exit
 - > 1+1
2
 - > 3+5
8
 - > 5-3
2
 - > 0
```

