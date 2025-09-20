# Transaction Decoder

Transaction Decoder is a CLI tool designed for parsing, decoding, and analyzing blockchain transactions. It provides utilities to interpret raw transaction data into human-readable formats, making it useful for developers, researchers, and anyone working with blockchain data.

## Features

- Decode raw blockchain transaction data.
- Support for legacy and segwit transactions.
- Output transaction details in a readable format.
- Easy integration as a CLI tool.
- Modular and extensible architecture for adding new decoders.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version recommended)
- Cargo (included with Rust installation)

### Installation

Clone the repository:

```bash
git clone https://github.com/arakinyemi/transaction-decoder.git
cd transaction-decoder
```



Build the project:

```bash
cargo build --release
```

### Usage

After building, run:

```bash
cargo run -- decode <raw_transaction_data>
```

Or use the built binary (after build):

```bash
./target/release/transaction-decoder decode <raw_transaction_data>
```


## Contact

For questions or feedback, open an issue on GitHub.

