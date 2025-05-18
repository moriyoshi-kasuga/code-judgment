{ pkgs ? import <nixpkgs> {} }:

let
  rustc182 = pkgs.buildEnv {
    name = "rustc182Env";
    paths = [ pkgs.rustc pkgs.gcc14 ];
  };
in
{
  inherit rustc182;
  go123 = pkgs.go_1_23;
  python313 = pkgs.python313;
}
