#!/bin/sh

COMMAND="$1"

shift

case "$COMMAND" in
test | t)
    RUST_LOG=chess-engine=info cargo test -- --nocapture "$@"
    ;;
debug | d)
    RUST_BACKTRACE=1 RUST_LOG=chess-engine=info ./target/debug/chess-engine "$@"
    ;;
run | r)
    cargo run --release "$@"
    ;;
build_nvidia | bn)
    cargo build --release --no-default-features --features cuda
    ;;
legal_masked|lm)
    ./target/release/chess-engine -l -m -p "~/chess-engine-stats/legal_masked/"
    ;;
legal_unmasked|lu)
    ./target/release/chess-engine -l -p "~/chess-engine-stats/legal_unmasked/"
    ;;
pseudo_legal_masked|pm)
    ./target/release/chess-engine -m -p "~/chess-engine-stats/pseudo_legal_masked/"
    ;;
pseudo_legal_unmasked|pu)
    ./target/release/chess-engine -p "~/chess-engine-stats/pseudo_legal_unmasked/"
    ;;
*)
    echo "Usage: $0 {test|t|debug|d|run|r}"
    ;;
esac
