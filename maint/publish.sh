#!/usr/bin/env bash
set -E

version="$(grep -E '^version = "(.*)"$' Cargo.toml | head -n1 | tail -c+12 | head -c-2)"
cargo build --release --target x86_64-pc-windows-gnu
git tag "$version"
git push --follow-tags
