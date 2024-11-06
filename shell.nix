{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    nodejs
    pkg-config
    openssl
    rustup
   ];

  shellHook = ''
    rustup default stable
    exec zsh
  '';
}