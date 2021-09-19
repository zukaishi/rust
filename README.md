- https://doc.rust-jp.rs/book-ja/ch01-01-installation.html
```
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
info: downloading installer

Welcome to Rust!

This will download and install the official compiler for the Rust
programming language, and its package manager, Cargo.

Rustup metadata and toolchains will be installed into the Rustup
home directory, located at:

  /Users/hishizuka/.rustup

This can be modified with the RUSTUP_HOME environment variable.

The Cargo home directory located at:

  /Users/hishizuka/.cargo

This can be modified with the CARGO_HOME environment variable.

The cargo, rustc, rustup and other commands will be added to
Cargo's bin directory, located at:

  /Users/hishizuka/.cargo/bin

This path will then be added to your PATH environment variable by
modifying the profile files located at:

  /Users/hishizuka/.profile
  /Users/hishizuka/.bash_profile
  /Users/hishizuka/.bashrc
  /Users/hishizuka/.zshenv

You can uninstall at any time with rustup self uninstall and
these changes will be reverted.

Current installation options:


   default host triple: x86_64-apple-darwin
     default toolchain: stable (default)
               profile: default
  modify PATH variable: yes

1) Proceed with installation (default)
2) Customize installation
3) Cancel installation
>1

info: profile set to 'default'
info: default host triple is x86_64-apple-darwin
info: syncing channel updates for 'stable-x86_64-apple-darwin'
info: latest update on 2021-09-09, rust version 1.55.0 (c8dfcfe04 2021-09-06)
info: downloading component 'cargo'
info: downloading component 'clippy'
info: downloading component 'rust-docs'
info: downloading component 'rust-std'
 21.0 MiB /  21.0 MiB (100 %)  16.4 MiB/s in  1s ETA:  0s
info: downloading component 'rustc'
 75.9 MiB /  75.9 MiB (100 %)  17.8 MiB/s in  4s ETA:  0s
info: downloading component 'rustfmt'
info: installing component 'cargo'
info: installing component 'clippy'
info: installing component 'rust-docs'
 17.1 MiB /  17.1 MiB (100 %)   3.8 MiB/s in  4s ETA:  0s
info: installing component 'rust-std'
 21.0 MiB /  21.0 MiB (100 %)  11.9 MiB/s in  1s ETA:  0s
info: installing component 'rustc'
 75.9 MiB /  75.9 MiB (100 %)  13.3 MiB/s in  5s ETA:  0s
info: installing component 'rustfmt'
info: default toolchain set to 'stable-x86_64-apple-darwin'

  stable-x86_64-apple-darwin installed - rustc 1.55.0 (c8dfcfe04 2021-09-06)


Rust is installed now. Great!

To get started you may need to restart your current shell.
This would reload your PATH environment variable to include
Cargo's bin directory ($HOME/.cargo/bin).

To configure your current shell, run:
source $HOME/.cargo/env
```
- ここでコンソール再起動。PATHエラーが起きていたので
```
rustc --version
rustc 1.55.0 (c8dfcfe04 2021-09-06)
```

```
cargo --version
cargo 1.55.0 (32da73ab1 2021-08-23)
```
