{ pkgs ? import <nixpkgs> {} }:

let
    install-rust = pkgs.writeShellScriptBin "install-rust" ''
    rustup install stable
    rustup default stable
    rustup component add rust-src
      '';
in

pkgs.mkShell {
  buildInputs = with pkgs; [
    # rustc
    # cargo
    # rustfmt
    # rustPackages.clippy
    # (import (builtins.fetchTarball https://github.com/NixOS/nixpkgs/archive/nixos-20.09-small.tar.gz) {}).rustracer
    rustup
    gdb
    cgdb

    install-rust
  ];
  RUST_BACKTRACE = 1;

  shellHook = ''
    export CARGO_HOME="$HOME/.local/share/rust/cargo"
    export RUSTUP_HOME="$HOME/.local/share/rust/rustup"
    export RUST_SRC_PATH="$HOME/.local/share/rust/rust-src"
    export PATH="$CARGO_HOME/bin:$RUSTUP_HOME/toolchains/stable-x86_64-unknown-linux-gnu/bin:$PATH"
  '';
}
