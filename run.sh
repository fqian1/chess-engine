#!/bin/sh

COMMAND="$1"

shift

case "$COMMAND" in
test | t)
    RUST_LOG=chess-engine=info cargo test -- --nocapture "$@"
    ;;
debug | d)
    RUST_BACKTRACE=1 RUST_LOG=chess-engine=info cargo run -- "$@"
    ;;
run | r)
    cargo run --release "$@"
    ;;
build_nvidia | bn)
    cargo build --release --no-default-features --features "cuda,autotune"
    ;;
quick|q)
    ./target/release/chess-engine -p -a "./tmp" -b 64 -n 80 -i 8 -e 16
    ;;
legal_masked|lm)
    ./target/release/chess-engine -l -m -p -a "./tmp/legal_masked/model.mpk" -g 512
    ;;
legal_unmasked|lu)
    ./target/release/chess-engine -l -p -a "./tmp/legal_unmasked/model.mpk" -g 512
    ;;
pseudo_legal_masked|pm)
    ./target/release/chess-engine -m -p -a "./tmp/pseudo_masked/model.mpk" -g 512
    ;;
pseudo_legal_unmasked|pu)
    ./target/release/chess-engine -p -a "./tmp/pseudo_unmasked/model.mpk" -g 512
    ;;
legal_unmasked_annealing|lua)
    ./target/release/chess-engine -l -p "./tmp/legal_unmasked_annealing/model.mpk" -g 512
    ;;
*)
    echo "Usage: $0 {test|t|debug|d|run|r}"
    ;;
esac
