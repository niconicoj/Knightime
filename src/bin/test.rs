use knightime::board::Board;

extern crate knightime;

fn main() {
    let mut board = Board::from_fen("8/8/5K2/8/8/6k1/7p/6N1 b - - 1 52").unwrap();

    let moves = board.generate_moves();

    println!("{}", moves);

    for mv in moves.into_iter() {
        board.make_move(mv, false);
        println!("{}", board);
        board.take_back_move();
    }
}
