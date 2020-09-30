# Rust CLI Calculator ![GitHub forks](https://img.shields.io/github/forks/JCSUCoder/rust-cli-calculator?style=social) ![GitHub stars](https://img.shields.io/github/stars/JCSUCoder/rust-cli-calculator?style=social) ![GitHub watchers](https://img.shields.io/github/watchers/JCSUCoder/rust-cli-calculator?style=social) ![GitHub](https://img.shields.io/github/license/JCSUCoder/rust-cli-calculator)
This a simple command line interface calculator that can take complex expresions with parentheses and give a result.

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
 - > (1/2)
0.5
 - > 1+(5-2)/(4+3)-8
-6.571429
 - > 2^(1/2)
1.414213
 - > 0
```

