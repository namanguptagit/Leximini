# LexiMini

A fast, minimal **Byte-Pair Encoding (BPE)** tokenizer implemented in **Rust**, with seamless Python bindings via **PyO3**.

##  Highlights

- **Fast** — Core tokenization logic written in Rust for maximum performance
- **Minimal** — Clean, readable implementation — great for learning how BPE works
- **Pythonic** — Drop-in Python module via native PyO3 extensions
- **Trainable** — Train your own BPE vocabulary directly from Python

## Quick Start

```bash
pip install leximini
```

```python
import leximini

# Initialize
tokenizer = leximini.get_encoding("gpt2")

# Train on your corpus
tokenizer.train("The quick brown fox jumps over the lazy dog.", 270)

# Encode & Decode
tokens = tokenizer.encode("The quick brown fox")
decoded = tokenizer.decode(tokens)
assert decoded == "The quick brown fox"
```

## Requirements

- **Python 3.8+**
- **Rust and Cargo** (for building from source): Install from [rustup.rs](https://rustup.rs/)

## Building from Source

From the `leximini` directory (the folder containing `pyproject.toml` and `Cargo.toml`):

```bash
pip install maturin
pip install .
```

Behind the scenes, the `maturin` build system will automatically invoke Cargo to compile the Rust extension and install it into your active Python environment.

## API Reference

### Initialization
```python
import leximini
tokenizer = leximini.get_encoding("gpt2")
```

### Training (BPE)
Train byte-pair merges from a sample corpus. Target vocabulary size must be ≥ 256 (base ASCII bytes).

```python
tokenizer.train("your training text here", 270)  # 256 base + 14 merges
```

### Encoding & Decoding
```python
tokens = tokenizer.encode("The quick brown fox")   # → list of ints
text = tokenizer.decode(tokens)                     # → original string
```

## License

MIT
