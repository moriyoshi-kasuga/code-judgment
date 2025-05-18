{ pkgs ? import <nixpkgs> {} }:

let
  rustc182Env = pkgs.buildEnv {
    name = "rustc182Env";
    paths = [ pkgs.rustc pkgs.gcc14 ];
  };
in
{
  inherit rustc182Env;
  go123Env = pkgs.go_1_23;
  python313Env = pkgs.python313;
}
