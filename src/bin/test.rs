use std::time::SystemTime;

use knightime::{board::Board, perft::Perft};

extern crate knightime;

fn main() {
    let board = Board::default();

    let mut perft = Perft::new(board);

    let start = SystemTime::now();

    perft.detailed_run(6, None);

    let duration = start.elapsed();

    match duration {
        Ok(d) => {
            println!("time for perft : {}", d.as_millis());
            println!("nodes : {}", perft.nodes);
            println!("captures : {}", perft.captures);
            println!("en passant : {}", perft.en_passants);
            println!("castling : {}", perft.castles);
            println!("promotion : {}", perft.promotions);
        }
        Err(_) => {}
    }
}
