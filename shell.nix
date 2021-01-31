{ pkgs ? import <nixpkgs> {} }:

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
  ];
  RUST_BACKTRACE = 1;

  shellHook = ''
    export CARGO_HOME="$PWD/.cargo"
    export RUSTUP_HOME="$PWD/.rustup"
    export RUST_SRC_PATH="$PWD/.rust-src"
    export PATH="$CARGO_HOME/bin:$RUSTUP_HOME/toolchains/stable-x86_64-unknown-linux-gnu/bin:$PATH"

    rustup install stable
    rustup default stable
    rustup component add rust-src
  '';
}
