{
  description = "Rust Nightly + CUDA Development Env";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ (import rust-overlay) ];
        config.allowUnfree = true;
      };
      rustNightly = pkgs.rust-bin.nightly.latest.default.override {
        extensions = [ "rust-src" "rust-analyzer" ];
      };
    in {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = [
          rustNightly
          pkgs.cargo-public-api
          pkgs.tinymist
          pkgs.cargo-watch
          pkgs.cudaPackages.cudatoolkit
          pkgs.linuxPackages.nvidia_x11
          pkgs.typstyle
          pkgs.flamegraph
          pkgs.heaptrack
        ];

        shellHook = ''
          export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath [
            pkgs.vulkan-loader
            pkgs.xorg.libX11
            pkgs.xorg.libXcursor
            pkgs.xorg.libXi
            pkgs.xorg.libXrandr
            pkgs.cudaPackages.cudatoolkit
            pkgs.linuxPackages.nvidia_x11
          ]}:/run/opengl-driver/lib:/run/opengl-driver-32/lib"

          export CUDA_PATH=${pkgs.cudaPackages.cudatoolkit}
          export EXTRA_LDFLAGS="-L/lib -L${pkgs.linuxPackages.nvidia_x11}/lib"
          export EXTRA_CCFLAGS="-I/usr/include"

          alias run='./run.sh'
          rustc --version
        '';
      };
    };
}
