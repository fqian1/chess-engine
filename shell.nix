let
  rust_overlay = import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
  pkgs = import <nixpkgs> {overlays = [rust_overlay];};
  rustNightly = pkgs.rust-bin.nightly.latest.default.override {
    extensions = ["rust-src" "rust-analyzer"];
  };
in
  pkgs.mkShell {
    buildInputs = [
      rustNightly
      pkgs.cargo
    ];

    shellHook = ''
      echo "Rust nightly shell loaded"
      rustc --version
    '';
  }
