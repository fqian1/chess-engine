let
  rust_overlay = import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
  pkgs = import <nixpkgs> {overlays = [rust_overlay];};
  rustNightly = pkgs.rust-bin.nightly.latest.default.override {
    extensions = ["rust-src" "rust-analyzer"];
  };
in
  pkgs.mkShell {
    buildInputs = [
      pkgs.rustup
      pkgs.pkg-config
      pkgs.openssl
      rustNightly
      pkgs.cargo
    ];
    # Weird thing to expose vulkan backend
    shellHook = ''
      LD_LIBRARY_PATH="''${LD_LIBRARY_PATH:+$LD_LIBRARY_PATH:}${
        with pkgs;
          lib.makeLibraryPath [
            vulkan-loader
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr
          ]
      }"
      export LD_LIBRARY_PATH
        echo "Rust nightly shell loaded"
        rustc --version
    '';
  }
