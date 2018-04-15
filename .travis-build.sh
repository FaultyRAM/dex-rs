#!/bin/sh
if [ "$RUSTFMT" = "true" ]; then
    cargo fmt -- --write-mode diff
else
    cargo build -vv --features "$FEATURES"
    cargo test -vv
    cargo doc -vv --no-deps
fi
