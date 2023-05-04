# Prettylog

This project demonstrates how to use the [log](https://crates.io/crates/log) crate in a [capsule](https://github.com/nervosnetwork/capsule) project.

Crate `log` uses atomic instructions [here](https://github.com/rust-lang/log/blob/ce9f217d584376646bac53dd9a2f9b9de7ab4be1/src/lib.rs#L347-L348). Since Rust does not provide a [target](https://doc.rust-lang.org/nightly/rustc/platform-support.html) for RV64IMC, this conditional compilation will always take effect. The compiled output contains atomic instructions, so it won't work on ckb2019 and ckb2021. Although there are some ways to solve this problem, almost all methods require manual modification of the source code.

The good news is that ckb2023 will introduce [RISC-V A instruction set](https://five-embeddev.com/riscv-isa-manual/latest/a.html) soon, which allows us to use crate `log` directly without modifying any source code. We have provided some preview tools for you to use for the upcoming ckb2023 in advance.

```sh
$ git clone https://github.com/nervosnetwork/ckb-standalone-debugger --branch ckb2023
$ cd ckb-standalone-debugger
# Since the upstream code is often rebased, we delete the lock file before compiling
$ rm Cargo.lock
$ cargo build

$ export PATH=$PATH:$(pwd)/target/build

```

```sh
$ capsule build
$ ckb-debugger --bin build/debug/prettylog
```

We can get an output that includes the log level, source code file, line number, and log content.

```text
[INFO  contracts/prettylog/src/entry.rs:7] Hello World!
```
