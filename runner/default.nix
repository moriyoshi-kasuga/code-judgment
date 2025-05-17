{ pkgs ? import <nixpkgs> {} }:

{
  rustc182Env = pkgs.rustc;
  go123Env = pkgs.go_1_23;
  python313Env = pkgs.python313;
}
