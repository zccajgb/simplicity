{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.nodejs
    pkgs.rustc
    pkgs.cargo
    pkgs.binutils
    pkgs.libiconv  # Add any other libraries you need
  ];

  shellHook = ''
    export RUST_SRC_PATH="${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}"
    # Additional environment setup can go here
  '';
}
