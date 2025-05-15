FROM nixos/nix:2.28.3

COPY default.nix /default.nix

RUN mkdir /runner

RUN nix-build /default.nix -A python313Env --out-link /runner/python313
RUN nix-build /default.nix -A python314Env --out-link /runner/python314
