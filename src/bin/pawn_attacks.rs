extern crate nightime;

use nightime::{defs::Piece, magic::find_magics};

fn main() {
    find_magics(Piece::Rook);
    find_magics(Piece::Bishop);
}
