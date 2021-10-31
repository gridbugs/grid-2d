# Nix shell with rust compiler and dependencies for libraries and examples

let
  moz_overlay_url = "https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz";
  moz_overlay = import (builtins.fetchTarball moz_overlay_url);
  nixpkgs = import <nixpkgs> {
    overlays = [ moz_overlay ];
  };
  ruststable = (nixpkgs.latest.rustChannels.stable.rust.override {
    extensions = [ "rust-src" "rust-analysis" ];
  });
in
with nixpkgs;
stdenv.mkDerivation rec {
  name = "moz_overlay_shell";
  buildInputs = [
    ruststable
  ];

  # Enable backtraces on panics
  RUST_BACKTRACE = 1;
}
