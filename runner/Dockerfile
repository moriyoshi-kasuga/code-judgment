FROM nixos/nix:2.28.3

COPY default.nix /default.nix
RUN mkdir /runner

RUN nix-build /default.nix -A rustc182Env --out-link /runner/rustc182Env
RUN nix-build /default.nix -A go123Env --out-link /runner/go123Env
RUN nix-build /default.nix -A python313Env --out-link /runner/python313Env

RUN nix-env -iA nixpkgs.time nixpkgs.nsjail

# NOTE: this is sample code. todo remove
# nsjail -Mo --chroot / --cgroup_mem_max 104 -- /root/.nix-profile/bin/echo "hey"
# nsjail -Mo --chroot / --cgroup_mem_max 104 --use_cgroupv2 -- /root/.nix-profile/bin/echo "hey"
