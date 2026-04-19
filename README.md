# 4 Round AES128 Square Attack Demonstration (Rust)

This project is an educational implementation demonstrating a **Square Attack** against a reduced 4-round version of **AES-128**.

## Project Overview

* **Algorithm:** AES (Advanced Encryption Standard)
* **Rounds:** 4 (reduced from the standard 10)
* **Key Length:** 128-bit (Provided by the user via string in code)
* **Input Size:** 128-bit (One byte provided by the user + [0u8; 15])
* **Language:** Rust
* **Purpose:** To visualize vulnerabilities in reduced-round ciphers and demonstrate the mathematical structure of AES.

## How the Square Attack Works

The Square Attack exploits the byte-oriented structure of AES. At only 4 rounds, the "diffusion" of the cipher is not yet strong enough to fully mask statistical dependencies.

### Attack Steps:

1.  **Constructing a $\Lambda$-set:**
    We create a set of 256 special plaintext blocks. In this set, one specific byte (the "active" byte) takes on all possible values from 0 to 255 across the blocks, while the other 15 bytes ("passive" bytes) remain constant (in this example constant 0x00).
    
2.  **Propagation through 3 Rounds:**
    Due to the algebraic properties of the SubBytes and MixColumns transformations, this set remains "balanced" after 3 full rounds. This means if you sum (XOR) the values of any byte position across all 256 states, the result is exactly zero.

3.  **Key Guessing (Round 4):**
    In the 4th round, this balance is disrupted. Since we have the final ciphertexts, we can guess a byte of the 4th round key (256 possibilities). 

4.  **Verification:**
    For each guess, we partially decrypt the ciphertext byte back through the 4th round's SubBytes step. If the XOR sum of these 256 decrypted values equals zero, the guessed key byte is almost certainly correct.

5.  **Extraction:**
    This is repeated for all 16 bytes until the full 4th-round key is recovered, which then allows the derivation of the original 128-bit master key.

## Installation & Usage

### Prerequisites
Ensure you have [Rust and Cargo](https://rustup.rs/) installed.

### Execution
When running the program, you will (not yet) be prompted to provide a **128-bit key as a string** (16 characters).

```bash
# Build the project
cargo build --release

# Run the project
cargo run
```

## Security Disclaimer
This implementation is for academic and research purposes only. While the Square Attack is highly effective against 4-round AES, full AES-128 (10 rounds) remains secure against this specific type of analysis.
