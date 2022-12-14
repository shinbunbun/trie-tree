{ pkgs ? import <nixpkgs> {} }:

let
in pkgs.mkShell {
  buildInputs = [ 
    pkgs.rustup
  ];
  shellHook = ''
  rustup component add rustfmt clippy
  rustup target add x86_64-unknown-linux-musl
  '';

#
  # Certain Rust tools won't work without this
  # This can also be fixed by using oxalica/rust-overlay and specifying the rust-src extension
  # See https://discourse.nixos.org/t/rust-src-not-found-and-other-misadventures-of-developing-rust-on-nixos/11570/3?u=samuela. for more details.
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
