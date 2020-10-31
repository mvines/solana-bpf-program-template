### Environment Setup
1. Get Rust from https://rustup.rs/
2. Install the Solana command-line tools from https://docs.solana.com/cli/install-solana-cli-tools#use-solanas-install-tool

### Build and test for program compiled natively
```
$ cargo build
$ cargo test
```

### Build and test the program compiled for BPF
```
$ cargo build-bpf
$ bpf=1 cargo test
```
