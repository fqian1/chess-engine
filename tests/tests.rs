use chess_engine::{self, ChessGame, ChessMove, ChessSquare, Color};
use log::{debug, info};

#[test]
fn move_generator_start_position() {
    env_logger::init();
    let mut chess_game = ChessGame::default();
    chess_game.make_move(&ChessMove::new(
        ChessSquare::from_name("a2").unwrap(),
        ChessSquare::from_name("a3").unwrap(),
        None,
    ));
    println!("{}", chess_game.position);

    assert_eq!(20, chess_game.position.pseudolegal_moves.iter().count())
}
//
// #[test]
// fn move_generator_kiwipete() {
//     let chess_game = ChessGame::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
//     let moves: Vec<ChessMove> =
//         chess_game.generate_pseudolegal().into_iter().filter(|x| chess_game.is_legal(x)).collect();
//     println!("{}", chess_game.chessboard.display_ascii());
//     print_moves(&moves);
//     assert_eq!(48, moves.iter().count());
// }
//
// #[test]
// fn move_generator_3() {
//     let chess_game = ChessGame::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1");
//     let moves: Vec<ChessMove> =
//         chess_game.generate_pseudolegal().into_iter().filter(|x| chess_game.is_legal(x)).collect();
//     // println!("{}", chess_game.chessboard.display_ascii());
//     // print_moves(&moves);
//     assert_eq!(14, moves.iter().count());
// }
//
// #[test]
// fn move_generator_4() {
//     let chess_game = ChessGame::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1");
//     let moves: Vec<ChessMove> =
//         chess_game.generate_pseudolegal().into_iter().filter(|x| chess_game.is_legal(x)).collect();
//     // println!("{}", chess_game.chessboard.display_ascii());
//     // print_moves(&moves);
//     assert_eq!(6, moves.iter().count());
// }
//
// #[test]
// fn test_lone_rook_center() {
//     let chess_game = ChessGame::from_fen("8/8/8/8/4R3/8/8/8 w - - 0 1");
//     let moves: Vec<ChessMove> =
//         chess_game.generate_pseudolegal().into_iter().collect();
//     println!("{}", chess_game.chessboard.display_ascii());
//     print_moves(&moves);
//     assert_eq!(14, moves.iter().count());
// }
