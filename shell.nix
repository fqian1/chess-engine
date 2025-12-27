{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  buildInputs = [
    pkgs.cargo
    pkgs.clippy
    pkgs.rustc
    pkgs.rust-analyzer
  ];
  shellHook = ''
    echo "rust shell loaded"
  '';
}
