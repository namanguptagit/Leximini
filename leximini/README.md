# LexiMini

A minimal byte-pair encoding (BPE) tokenizer implementation in Rust, with Python bindings via PyO3.

## Requirements

To install and build this package from source, you will need:
- **Python 3.7+**
- **Rust and Cargo**: Install from [rustup.rs](https://rustup.rs/)

## Installation

Because this library includes native Rust code, it needs to be compiled when installed.

From the `leximini` directory (the folder containing this `pyproject.toml` and `Cargo.toml`), simply run:

```bash
pip install .
```

Behind the scenes, the `maturin` build system will automatically invoke Cargo to compile the Rust extension and install it seamlessly into your active Python environment.

## Usage

Once installed, you can import and use it in Python just like `tiktoken`.

### 1. Initialization
```python
import leximini

# Initialize the tokenizer
tokenizer = leximini.get_encoding("gpt2")
```

### 2. Training (BPE)
The tokenizer needs to learn byte-pair merges from a sample corpus. Provide a text string and your target vocabulary size (must be >= 256 for the base ASCII bytes).

```python
sample_text = "The quick brown fox jumps over the lazy dog."
target_vocab_size = 270 # 256 base bytes + 14 merged tokens

tokenizer.train(sample_text, target_vocab_size)
```

### 3. Encoding and Decoding
Once trained, use `encode` to compress text into token IDs, and `decode` to reconstruct the string losslessly.

```python
text_to_encode = "The quick brown fox"

# Encode
tokens = tokenizer.encode(text_to_encode)
print(tokens) # Output will be a list of integers, e.g. [84, 104, 101, 260, ...]

# Decode
decoded_text = tokenizer.decode(tokens)
assert decoded_text == text_to_encode
```
