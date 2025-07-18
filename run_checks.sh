#!/bin/bash
# run_and_copy.sh

(
  cargo clippy --all-targets --all-features -- -D warnings 2>&1
  cat src/*.rs
  cat src/ui/*.rs
  cat Cargo.toml
) | tee >(pbcopy)
