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
quick|q)
    ./target/release/chess-engine -p "./tmp" -b 8 -n 80 -i 8 -e 40
legal_masked|lm)
    ./target/release/chess-engine -l -m -p "/scratch/fq00038-legal_masked/"
    ;;
legal_unmasked|lu)
    ./target/release/chess-engine -l -p "/scratch/fq00038-legal_unmasked/"
    ;;
pseudo_legal_masked|pm)
    ./target/release/chess-engine -m -p "/scratch/fq0038-pseudo_legal_masked/"
    ;;
pseudo_legal_unmasked|pu)
    ./target/release/chess-engine -p "/scratch/fq00038-pseudo_legal_unmasked/"
    ;;
*)
    echo "Usage: $0 {test|t|debug|d|run|r}"
    ;;
esac
