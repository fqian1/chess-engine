#!/bin/bash

BIN_DIR="$HOME/.local/bin"
SF_DOWNLOAD_URL="https://github.com/official-stockfish/Stockfish/releases/latest/download/stockfish-ubuntu-x86-64-avx2.tar"

mkdir -p "$BIN_DIR"

echo "Downloading Stockfish..."
curl -L "$SF_DOWNLOAD_URL" -o stockfish.tar

echo "Extracting..."
tar -xf stockfish.tar

mv stockfish/stockfish-ubuntu-x86-64-avx2 "$BIN_DIR/stockfish"
chmod 755 "$BIN_DIR/stockfish"

rm -rf stockfish stockfish.tar

# Add to PATH only if not already present to prevent duplicates
if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
fi

echo "Success! Stockfish is installed at $BIN_DIR/stockfish"
echo "Please run 'source ~/.bashrc' to update your current session."
