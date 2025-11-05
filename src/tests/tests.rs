use std::collections::HashMap; // Assuming ChessGame uses this

mod tests {
    use super::*;

    // Helper function to create a square for tests, panics on failure.
    fn sq(name: &str) -> ChessSquare {
        ChessSquare::from_name(name).expect("Failed to create square in test")
    }

    #[test]
    fn test_color_logic() {
        assert_eq!(Color::White.opposite(), Color::Black);
        assert_eq!(Color::Black.opposite(), Color::White);

        assert_eq!(Color::from_char('w'), Some(Color::White));
        assert_eq!(Color::from_char('b'), Some(Color::Black));
        assert_eq!(Color::from_char('W'), None); // Assuming case-sensitive
        assert_eq!(Color::from_char('x'), None);

        assert_eq!(Color::White.to_char(), 'w');
        assert_eq!(Color::Black.to_char(), 'b');
    }

    #[test]
    fn test_piece_type_logic() {
        assert_eq!(PieceType::from_idx(0), Some(PieceType::Pawn));
        assert_eq!(PieceType::from_idx(5), Some(PieceType::King));
        assert_eq!(PieceType::from_idx(6), None);

        assert_eq!(PieceType::from_char('P'), Some(PieceType::Pawn));
        assert_eq!(PieceType::from_char('n'), Some(PieceType::Knight));
        assert_eq!(PieceType::from_char('k'), Some(PieceType::King));
        assert_eq!(PieceType::from_char('X'), None);

        assert_eq!(PieceType::Queen.to_char(Color::White), 'Q');
        assert_eq!(PieceType::Pawn.to_char(Color::Black), 'p');
    }

    #[test]
    fn test_chess_square_creation_and_properties() {
        // Valid creation
        assert!(ChessSquare::new(0).is_some());
        assert!(ChessSquare::new(63).is_some());
        assert!(ChessSquare::from_coords(0, 0).is_some()); // a1
        assert!(ChessSquare::from_coords(7, 7).is_some()); // h8
        assert!(ChessSquare::from_name("a1").is_some());
        assert!(ChessSquare::from_name("h8").is_some());

        // Invalid creation
        assert!(ChessSquare::new(64).is_none());
        assert!(ChessSquare::from_coords(8, 0).is_none());
        assert!(ChessSquare::from_coords(0, 8).is_none());
        assert!(ChessSquare::from_name("i1").is_none());
        assert!(ChessSquare::from_name("a9").is_none());
        assert!(ChessSquare::from_name("").is_none());
        assert!(ChessSquare::from_name("e2e4").is_none());

        // Properties
        let e4 = sq("e4");
        assert_eq!(e4.index(), 28);
        assert_eq!(e4.file(), 4);
        assert_eq!(e4.rank(), 3);
        assert_eq!(e4.to_name(), "e4");
        assert_eq!(e4.name(), "e4"); // Assuming this is an alias for to_name
    }

    #[test]
    fn test_chess_move_uci_parsing() {
        // Standard move
        let mov = ChessMove::from_uci("e2e4").unwrap();
        assert_eq!(mov.from, sq("e2"));
        assert_eq!(mov.to, sq("e4"));
        assert_eq!(mov.promotion, None);

        // Promotion move
        let mov_promo = ChessMove::from_uci("a7a8q").unwrap();
        assert_eq!(mov_promo.from, sq("a7"));
        assert_eq!(mov_promo.to, sq("a8"));
        assert_eq!(mov_promo.promotion, Some(PieceType::Queen));

        // Case-insensitivity for promotion piece? Let's assume it's required.
        let mov_promo_upper = ChessMove::from_uci("h2h1R").unwrap();
        assert_eq!(mov_promo_upper.promotion, Some(PieceType::Rook));

        // Invalid moves
        assert!(ChessMove::from_uci("e2e9").is_err()); // Invalid square
        assert!(ChessMove::from_uci("e2").is_err()); // Too short
        assert!(ChessMove::from_uci("e2e4e5").is_err()); // Too long
        assert!(ChessMove::from_uci("a7a8x").is_err()); // Invalid promotion piece
    }

    #[test]
    fn test_chess_move_uci_serialization() {
        let mov = ChessMove::new(sq("g1"), sq("f3"));
        assert_eq!(mov.to_uci(), "g1f3");

        let mut promo_mov = ChessMove::new(sq("b7"), sq("b8"));
        promo_mov.promotion = Some(PieceType::Knight);
        assert_eq!(promo_mov.to_uci(), "b7b8n");
    }

    #[test]
    fn test_bitboard_operations() {
        let mut bb = Bitboard::EMPTY();
        assert!(bb.is_empty());
        assert_eq!(bb.count_ones(), 0);

        let a1 = sq("a1");
        let h8 = sq("h8");

        bb.set(a1);
        assert!(!bb.is_empty());
        assert!(bb.is_set(a1));
        assert!(!bb.is_set(h8));
        assert_eq!(bb.count_ones(), 1);
        assert_eq!(bb.lsb_square(), Some(a1));
        assert_eq!(bb.msb_square(), Some(a1));

        bb.set(h8);
        assert_eq!(bb.count_ones(), 2);
        assert!(bb.is_set(a1));
        assert!(bb.is_set(h8));
        assert_eq!(bb.lsb_square(), Some(a1));
        assert_eq!(bb.msb_square(), Some(h8));

        let popped = bb.pop_lsb();
        assert_eq!(popped, Some(a1));
        assert_eq!(bb.count_ones(), 1);
        assert!(!bb.is_set(a1));
        assert!(bb.is_set(h8));

        bb.clear(h8);
        assert!(bb.is_empty());
    }

    #[test]
    fn test_bitboard_set_logic() {
        let bb1 = Bitboard::from_square(sq("e4"));
        let bb2 = Bitboard::from_square(sq("d5"));
        let bb3 = Bitboard::from_square(sq("e4"));

        // Union
        let union_bb = bb1.union(bb2);
        assert!(union_bb.is_set(sq("e4")));
        assert!(union_bb.is_set(sq("d5")));
        assert_eq!(union_bb.count_ones(), 2);

        // Intersection
        let intersect_bb = union_bb.intersection(bb1);
        assert!(intersect_bb.is_set(sq("e4")));
        assert!(!intersect_bb.is_set(sq("d5")));
        assert_eq!(intersect_bb.count_ones(), 1);

        let no_intersect = bb1.intersection(bb2);
        assert!(no_intersect.is_empty());

        // Difference
        let diff_bb = union_bb.difference(bb1);
        assert!(!diff_bb.is_set(sq("e4")));
        assert!(diff_bb.is_set(sq("d5")));
        assert_eq!(diff_bb.count_ones(), 1);
    }

    #[test]
    fn test_bitboard_shifting() {
        let d4 = Bitboard::from_square(sq("d4"));

        assert_eq!(d4.shift_north().lsb_square(), Some(sq("d5")));
        assert_eq!(d4.shift_south().lsb_square(), Some(sq("d3")));
        assert_eq!(d4.shift_east().lsb_square(), Some(sq("e4")));
        assert_eq!(d4.shift_west().lsb_square(), Some(sq("c4")));

        // Edge cases
        let a1 = Bitboard::from_square(sq("a1"));
        assert!(a1.shift_south().is_empty());
        assert!(a1.shift_west().is_empty());

        let h8 = Bitboard::from_square(sq("h8"));
        assert!(h8.shift_north().is_empty());
        assert!(h8.shift_east().is_empty());
    }

    #[test]
    fn test_castling_rights() {
        let mut rights = CastlingRights::from_fen("KQkq");
        assert!(rights.has(CastlingRights::WHITE_KINGSIDE));
        assert!(rights.has(CastlingRights::WHITE_QUEENSIDE));
        assert!(rights.has(CastlingRights::BLACK_KINGSIDE));
        assert!(rights.has(CastlingRights::BLACK_QUEENSIDE));
        assert_eq!(rights.to_fen(), "KQkq");

        rights.remove(CastlingRights::WHITE_KINGSIDE);
        assert!(!rights.has(CastlingRights::WHITE_KINGSIDE));
        assert!(rights.has(CastlingRights::WHITE_QUEENSIDE));
        assert_eq!(rights.to_fen(), "Qkq");

        let rights_none = CastlingRights::from_fen("-");
        assert!(!rights_none.has(CastlingRights::WHITE_KINGSIDE));
        assert_eq!(rights_none.to_fen(), "-");
    }

    #[test]
    fn test_board_piece_manipulation() {
        let mut board = ChessBoard::empty();
        let white_pawn = ChessPiece {
            color: Color::White,
            piece_type: PieceType::Pawn,
        };
        let black_king = ChessPiece {
            color: Color::Black,
            piece_type: PieceType::King,
        };
        let e4 = sq("e4");
        let d8 = sq("d8");

        // Add piece
        board.add_piece(white_pawn, e4);
        assert_eq!(board.get_piece_at(e4), Some(white_pawn));
        assert!(board.white_occupancy.is_set(e4));
        assert!(board.all_pieces.is_set(e4));
        assert!(!board.black_occupancy.is_set(e4));
        assert_eq!(
            board
                .get_piece_bitboard(Color::White, PieceType::Pawn)
                .lsb_square(),
            Some(e4)
        );

        // Move piece
        let e5 = sq("e5");
        board.move_piece(e4, e5, white_pawn);
        assert_eq!(board.get_piece_at(e4), None);
        assert_eq!(board.get_piece_at(e5), Some(white_pawn));
        assert!(!board.white_occupancy.is_set(e4));
        assert!(board.white_occupancy.is_set(e5));

        // Remove piece
        board.remove_piece(white_pawn, e5);
        assert_eq!(board.get_piece_at(e5), None);
        assert!(board.white_occupancy.is_empty());
        assert!(board.all_pieces.is_empty());
    }

    #[test]
    fn test_game_fen_parsing_startpos() {
        let start_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let game = ChessGame::from_fen(start_fen);

        assert_eq!(game.side_to_move, Color::White);
        assert!(game.castling_rights.has(CastlingRights::WHITE_KINGSIDE));
        assert!(game.castling_rights.has(CastlingRights::BLACK_QUEENSIDE));
        assert_eq!(game.en_passant, None);
        assert_eq!(game.halfmove_clock, 0);
        assert_eq!(game.fullmove_counter, 1);

        // Check a few pieces
        let white_rook = Some(ChessPiece {
            color: Color::White,
            piece_type: PieceType::Rook,
        });
        let black_pawn = Some(ChessPiece {
            color: Color::Black,
            piece_type: PieceType::Pawn,
        });
        assert_eq!(game.board.get_piece_at(sq("a1")), white_rook);
        assert_eq!(game.board.get_piece_at(sq("d7")), black_pawn);
        assert_eq!(game.board.get_piece_at(sq("e4")), None);
    }

    #[test]
    fn test_game_fen_parsing_complex() {
        // After 1. e4 c5 2. Nf3
        let fen = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2";
        let game = ChessGame::from_fen(fen);

        assert_eq!(game.side_to_move, Color::Black);
        assert!(game.castling_rights.has(CastlingRights::WHITE_KINGSIDE));
        assert!(game.castling_rights.has(CastlingRights::BLACK_KINGSIDE));
        assert_eq!(game.en_passant, None); // No en passant square in this FEN
        assert_eq!(game.halfmove_clock, 1);
        assert_eq!(game.fullmove_counter, 2);

        // Check a few pieces
        let white_knight = Some(ChessPiece {
            color: Color::White,
            piece_type: PieceType::Knight,
        });
        assert_eq!(game.board.get_piece_at(sq("f3")), white_knight);
        assert_eq!(game.board.get_piece_at(sq("e2")), None);
    }

    #[test]
    fn test_game_fen_serialization() {
        // Note: FEN serialization can be tricky. A common difference is ` ` vs ` e3 ` for en passant.
        // This test assumes the output is canonical.
        let fen1 = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let game1 = ChessGame::from_fen(fen1);
        assert_eq!(game1.to_fen(), fen1);

        let fen2 = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2";
        let game2 = ChessGame::from_fen(fen2);
        assert_eq!(game2.to_fen(), fen2);

        let fen3_with_ep = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
        let game3 = ChessGame::from_fen(fen3_with_ep);
        assert_eq!(game3.to_fen(), fen3_with_ep);
    }

    #[test]
    fn test_make_move() {
        let start_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let mut game = ChessGame::from_fen(start_fen);

        // 1. e4
        let mv = ChessMove::from_uci("e2e4").unwrap();
        game.make_move(&mv);

        // Check state after move
        assert_eq!(game.side_to_move, Color::Black);
        assert_eq!(game.board.get_piece_at(sq("e2")), None);
        let white_pawn = Some(ChessPiece {
            color: Color::White,
            piece_type: PieceType::Pawn,
        });
        assert_eq!(game.board.get_piece_at(sq("e4")), white_pawn);
        assert_eq!(game.en_passant, Some(sq("e3")));
        assert_eq!(game.halfmove_clock, 0); // Pawn move resets clock
        assert_eq!(game.fullmove_counter, 1); // Still on move 1

        // 1... c5
        let mv2 = ChessMove::from_uci("c7c5").unwrap();
        game.make_move(&mv2);

        // Check state after second move
        assert_eq!(game.side_to_move, Color::White);
        assert_eq!(game.en_passant, Some(sq("c6")));
        assert_eq!(game.fullmove_counter, 2); // Now on move 2

        // Check that FEN reflects the new position
        let expected_fen = "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2";
        assert_eq!(game.to_fen(), expected_fen);
    }

    #[test]
    fn test_make_move_updates_castling_rights() {
        // A position where rooks and kings can move
        let fen = "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1";
        let mut game = ChessGame::from_fen(fen);

        // Move white king, should lose both white castling rights
        game.make_move(&ChessMove::from_uci("e1d1").unwrap());
        assert!(!game.castling_rights.has(CastlingRights::WHITE_KINGSIDE));
        assert!(!game.castling_rights.has(CastlingRights::WHITE_QUEENSIDE));
        assert!(game.castling_rights.has(CastlingRights::BLACK_KINGSIDE)); // Black rights unaffected

        // Reset and move a1 rook
        let mut game = ChessGame::from_fen(fen);
        game.make_move(&ChessMove::from_uci("a1a2").unwrap());
        assert!(game.castling_rights.has(CastlingRights::WHITE_KINGSIDE));
        assert!(!game.castling_rights.has(CastlingRights::WHITE_QUEENSIDE));
    }
}
