{ pkgs ? import <nixpkgs> {} }:

let
  rust182 = pkgs.buildEnv {
    name = "rustc182";
    paths = [ pkgs.rustc pkgs.gcc14 ];
  };
in
{
  inherit rust182;
  go123 = pkgs.go_1_23;
  python313 = pkgs.python313;
}
