use knightime::board::Board;

extern crate knightime;

fn main() {
    let mut board =
        Board::from_fen("r3k2r/ppp2pbp/2nqpnp1/1B1pNb2/3P4/P1N1P3/1PPBQPPP/R3K2R b KQkq - 4 9")
            .unwrap();

    let moves = board.generate_moves();

    println!("{}", moves);

    for mv in moves.into_iter() {
        board.make_move(mv, false);
        println!("{}", board);
        board.take_back_move();
    }
}
