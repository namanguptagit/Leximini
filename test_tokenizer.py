import leximini

print("Testing leximini...")

# Mimic the gpt2 get_encoding call from tiktoken
tokenizer = leximini.get_encoding("gpt2")

text = (
    "Hello, do you like tea? <|endoftext|> In the sunlit terraces"
     "of someunknownPlace."
)

integers = tokenizer.encode(text, allowed_special={"<|endoftext|>"})

print("Encoded:", integers)

strings = tokenizer.decode(integers)

print("Decoded:", strings)

assert strings == text, "Decoded string does not match original!"
print("Success! Decoded string matches original text.")
