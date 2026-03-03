# LexiMini

LexiMini is a fast, minimal Byte-Pair Encoding (BPE) tokenizer implemented in Rust, with out-of-the-box Python bindings powered by PyO3. It is designed to be lightweight, easy to understand, and highly performant, providing an interface similar to OpenAI's `tiktoken`.

## Features
- **Fast:** Core tokenization logic is written in Rust.
- **Minimal:** Easy to read and understand if you want to learn how BPE works under the hood.
- **Pythonic:** Seamlessly integrates into your Python projects via PyO3 native extensions.
- **Trainable:** Train your own BPE merges directly from Python.

---

##  Quick Start (Python)

You can easily install the pre-compiled version using `pip`:

### macOS & Linux
```bash
pip3 install leximini
```

### Windows
```powershell
pip install leximini
```

### Basic Usage

Once installed, you can train a tokenizer on your own text, and encode/decode strings:

```python
import leximini

# 1. Initialize the tokenizer
tokenizer = leximini.get_encoding("gpt2")

# 2. Train the tokenizer
sample_text = "The quick brown fox jumps over the lazy dog. The fox is fast."
target_vocab_size = 270 # 256 base ASCII bytes + 14 merged tokens
tokenizer.train(sample_text, target_vocab_size)

# 3. Encode text into token IDs
text_to_encode = "The quick brown fox"
tokens = tokenizer.encode(text_to_encode)
print("Tokens:", tokens) # e.g. [84, 104, 101, 260, ...]

# 4. Decode token IDs back to text
decoded_text = tokenizer.decode(tokens)
print("Decoded:", decoded_text)
assert decoded_text == text_to_encode
```

---

## 🛠️ Building from Source

If you want to modify the source code or build the extension yourself locally, you will need:
- **Python 3.8+**
- **Rust and Cargo**: Install from [rustup.rs](https://rustup.rs/)

### Instructions

1. Clone this repository:
   ```bash
   git clone https://github.com/namanguptagit/leximini.git
   cd leximini/leximini
   ```

2. Install `maturin`, the build system for PyO3 extensions:
   ```bash
   pip install maturin
   ```

3. Build and install the package into your current Python environment:
   ```bash
   python -m maturin develop --release
   # Or using pip directly:
   # pip install .
   ```

## Repository Structure

- `/leximini`: Contains the Rust source code (`src/`), the `Cargo.toml`, and the Python configuration (`pyproject.toml`).
  - `src/lib.rs`: The PyO3 bindings and Rust BPE implementation.
  - `test_bpe.py`: Python script for testing the tokenizer functionality.

## License

MIT License
