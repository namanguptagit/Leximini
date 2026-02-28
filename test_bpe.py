import leximini

print("Testing BPE leximini...")

# Initialize
tokenizer = leximini.get_encoding("gpt2")

training_text = """
The Byte Pair Encoding algorithm is a data compression technique that iteratively replaces the most common pair of consecutive bytes or characters with a single, unused byte. It is widely used in modern Large Language Models and natural language processing tasks because it elegantly balances vocabulary size and sequence length without requiring manual linguistic rules.
"""

# Let's target a vocab size of 300 (256 base characters + 44 new merged tokens)
target_vocab_size = 300
print(f"Training tokenizer to vocab size {target_vocab_size}...")
tokenizer.train(training_text, target_vocab_size)

test_text = "Byte Pair Encoding is elegantly used in large language models!"

# Encode
print("\nEncoding new string:")
print(f"Original: '{test_text}'")
encoded_tokens = tokenizer.encode(test_text)
print("Encoded tokens:", encoded_tokens)

# Verify some tokens were merged (token IDs > 255 exist)
merged_count = sum(1 for t in encoded_tokens if t > 255)
print(f"Number of merged tokens applied: {merged_count} out of {len(encoded_tokens)} total tokens")

# Decode
decoded_string = tokenizer.decode(encoded_tokens)
print("\nDecoded:", decoded_string)

assert decoded_string == test_text, "Decoded string does not match original!"
print("\nSuccess! Decoded string matches original text.")
