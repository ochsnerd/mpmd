#!/usr/bin/env bash
# Sets up direnv + nix-direnv so the flake's dev shell loads automatically in the devcontainer.
set -euo pipefail

# Make nix available in this (non-login) shell
. /nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh

# Install direnv and nix-direnv (caches the flake's dev shell)
nix profile install nixpkgs#direnv nixpkgs#nix-direnv

mkdir -p ~/.config/direnv
echo 'source $HOME/.nix-profile/share/nix-direnv/direnvrc' > ~/.config/direnv/direnvrc

# Hook direnv into bash and zsh
echo 'eval "$(direnv hook bash)"' >> ~/.bashrc
echo 'eval "$(direnv hook zsh)"' >> ~/.zshrc

# Trust .envrc and build the dev shell once up front
direnv allow .
direnv exec . true
