use knightime::board::Board;
use knightime::board::MakeMoveError;

extern crate knightime;

fn main() {
    let mut board =
        Board::from_fen("r3k2r/ppp2pbp/2nqp1p1/1B1pNb2/1P1P4/P1NQP1n1/2PB1PPP/R3K2R b KQkq - 2 11")
            .unwrap();

    let moves = board.generate_moves();

    println!("{}", moves);

    for mv in moves.into_iter() {
        match board.make_move(mv, false) {
            Ok(_) => {
                board.take_back_move();
            }
            Err(MakeMoveError::IllegalMove(illegal_move)) => {
                println!("illegal move : {}", illegal_move);
            }
            _ => {}
        };
    }
}
