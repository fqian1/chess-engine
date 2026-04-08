let
  rust_overlay = import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
  pkgs = import <nixpkgs> {
    overlays = [rust_overlay];
    config.allowUnfree = true;
  };
  rustNightly = pkgs.rust-bin.nightly.latest.default.override {
    extensions = ["rust-src" "rust-analyzer"];
  };
in
  pkgs.mkShell {
    buildInputs = [
      rustNightly
      pkgs.cargo-public-api
      pkgs.cargo-watch
      pkgs.cudaPackages.cudatoolkit 
      pkgs.linuxPackages.nvidia_x11 
      pkgs.typstyle
    ];

    shellHook = ''
      export LD_LIBRARY_PATH="''${LD_LIBRARY_PATH:+$LD_LIBRARY_PATH:}${
        with pkgs;
          lib.makeLibraryPath [
            vulkan-loader
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr
            cudaPackages.cudatoolkit
            linuxPackages.nvidia_x11
          ]
      }:/run/opengl-driver/lib:/run/opengl-driver-32/lib"

      export CUDA_PATH=${pkgs.cudaPackages.cudatoolkit}
      export EXTRA_LDFLAGS="-L/lib -L${pkgs.linuxPackages.nvidia_x11}/lib"
      export EXTRA_CCFLAGS="-I/usr/include"

      alias run='./run.sh'
      echo "Rust nightly shell with CUDA support loaded"
      rustc --version
      nvcc --version
    '';
  }
