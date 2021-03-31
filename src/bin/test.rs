use knightime::board::Board;

extern crate knightime;

fn main() {
    let mut board =
        Board::from_fen("6k1/pr3pp1/1p3q1p/2p5/3n4/P1BP3P/1P3PP1/R1Q3K1 b - - 1 24").unwrap();

    let moves = board.generate_moves();

    println!("{}", moves);

    for mv in moves.into_iter() {
        board.make_move(mv, false);
        println!("{}", board);
        board.take_back_move();
    }
}
