{ pkgs ? import <nixpkgs> {} }:

{
  python313Env = pkgs.python313;
  python314Env = pkgs.python314;
}
