# Chess Client TODO List

## 1. Move Generation and Validation

- [ ] Implement legal move generation for all piece types (pawns, knights, bishops, rooks, queens, kings).
- [ ] Include special moves:
    - [ ] Castling (kingside and queenside).
    - [ ] En passant.
    - [ ] Pawn promotion.
- [ ] Validate moves against the generated legal moves.
- [ ] Consider pins, checks, and other chess rules in move generation.
- [ ] The `validate_move` function in `chess_game.rs` should be updated or replaced with this new logic.

## 2. Check, Checkmate, and Stalemate Detection

- [ ] After each move, determine if the opponent is in check.
- [ ] Implement checkmate detection to end the game.
- [ ] Implement stalemate detection.
- [ ] Update the game loop in `main.rs` to handle these game-ending conditions.

## 3. User Interface Improvements

- [ ] Display captured pieces for both sides.
- [ ] Announce the game result (e.g., "Checkmate! White wins.", "Stalemate. The game is a draw.").
- [ ] Consider a more interactive way to input moves, such as highlighting squares (optional, advanced).

## 4. Additional Commands

- [ ] `new`: Start a new game.
- [ ] `undo`: Undo the last move.
- [ ] `fen`: Print the FEN string of the current position.
- [ ] `pgn`: Export the game to PGN format.

## 5. Testing

- [ ] Create a `tests` directory with a `tests.rs` file if it doesn't exist.
- [ ] Add unit tests for:
    - [ ] Move generation for each piece.
    - [ ] FEN parsing and generation.
    - [ ] Check, checkmate, and stalemate detection.
    - [ ] Special moves (castling, en passant, promotion).
- [ ] Add integration tests for a full game or a series of moves.

## 6. Refactoring and Code Quality

- [ ] Split the move generation logic from `chess_game.rs` into its own module (e.g., `move_gen.rs`).
- [ ] Remove unused imports from `main.rs` and other files.
- [ ] Add comments to explain complex logic, especially in move generation and game state management.
- [ ] Ensure consistent code formatting and style.
