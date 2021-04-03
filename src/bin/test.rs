use knightime::{
    board::Board, communication::uci::parse_uci_position, constants::*, defs::*,
    move_generator::movelist::Move,
};

extern crate knightime;

fn main() {
    let uci_position =
            "position fen r1bqkbnr/pp1p1ppp/2n1p3/2p5/2BPP3/5N2/PPP2PPP/RNBQK2R b KQkq d3 0 4 moves d7d5 c4b5";
    let uci_board = parse_uci_position(uci_position).unwrap();

    let mut target_board =
        Board::from_fen("r1bqkbnr/pp1p1ppp/2n1p3/2p5/2BPP3/5N2/PPP2PPP/RNBQK2R b KQkq d3 0 4")
            .unwrap();
    let mv = Move::new(D7, D5, Piece::Pawn, None, false, true, false, false);
    target_board.make_move(mv, false).unwrap();
    let mv = Move::new(C4, D5, Piece::Bishop, None, false, false, false, false);
    target_board.make_move(mv, false).unwrap();

    println!("{}", uci_board);
    println!("{}", target_board);
}
