Environment Setup:
1. Get Rust from https://rustup.rs/
2. Install the Solana command-line tools with `sh -c "$(curl -sSfL https://release.solana.com/v1.4.4/install)"`

Build and test for program compiled natively:
```
$ cargo build
$ cargo test
```

Build and test the program compiled for BPF:
```
$ cargo build-bpf
$ bpf=1 cargo test
```
