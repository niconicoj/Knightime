use crate::{
    bitboard::Bitboard,
    defs::{Piece, Side},
    magic::{Magic, BISHOP_MAGIC_NUMBERS, BISHOP_TABLE_SIZE, ROOK_MAGIC_NUMBERS, ROOK_TABLE_SIZE},
};

use super::MoveGenerator;

impl MoveGenerator {
    pub fn init_pawns_attacks(side: Side) -> Vec<Bitboard> {
        let mut pawn_attacks: Vec<Bitboard> = vec![];
        for square in 0..64 {
            pawn_attacks.push(Self::mask_pawn_attacks(side, square));
        }
        pawn_attacks
    }

    pub fn init_knights_attacks() -> Vec<Bitboard> {
        let mut knight_attacks: Vec<Bitboard> = vec![];
        for square in 0..64 {
            knight_attacks.push(Self::mask_knight_attacks(square));
        }
        knight_attacks
    }

    pub fn init_king_attacks() -> Vec<Bitboard> {
        let mut king_attacks: Vec<Bitboard> = vec![];
        for square in 0..64 {
            king_attacks.push(Self::mask_king_attacks(square));
        }
        king_attacks
    }

    pub fn init_magics(&mut self, piece: Piece) {
        let mut offset = 0;

        for sq in 0..64 {
            let mask = match piece {
                Piece::Rook => MoveGenerator::mask_rook_attacks(sq),
                Piece::Bishop => MoveGenerator::mask_bishop_attacks(sq),
                _ => panic!(
                    "can only generate magic for rook or bishop, {:?} was passed.",
                    piece
                ),
            };

            let bits = mask.get_value().count_ones(); // Number of set bits in the mask
            let permutations = 2u64.pow(bits); // Number of blocker boards to be indexed.
            let end = offset + permutations - 1; // End point in the attack table.
            let blocker_boards = mask.get_blocker_boards();

            let attack_boards = match piece {
                Piece::Rook => MoveGenerator::generate_rook_attack_boards(sq, &blocker_boards),
                Piece::Bishop => MoveGenerator::generate_bishop_attack_boards(sq, &blocker_boards),
                _ => panic!(
                    "can only generate magic for rook or bishop, {:?} was passed.",
                    piece
                ),
            };

            let mut magic: Magic = Default::default();

            magic.mask = mask;
            magic.shift = (64 - bits) as u8;
            magic.offset = offset;
            magic.nr = match piece {
                Piece::Bishop => BISHOP_MAGIC_NUMBERS[sq as usize],
                Piece::Rook => ROOK_MAGIC_NUMBERS[sq as usize],
                _ => panic!(
                    "can only generate magic for rook or bishop, {:?} was passed.",
                    piece
                ),
            };

            for i in 0..permutations {
                let next = i as usize;
                let index = magic.get_index(blocker_boards[next]);

                let table = match piece {
                    Piece::Rook => &mut self.rooks[..],
                    Piece::Bishop => &mut self.bishops[..],
                    _ => panic!(
                        "can only generate magic for rook or bishop, {:?} was passed.",
                        piece
                    ),
                };

                if table[index] == 0 {
                    let fail_low = index < offset as usize;
                    let fail_high = index > end as usize;
                    assert!(!fail_low && !fail_high, "Indexing error. Error in Magics.");
                    table[index] = attack_boards[next];
                } else {
                    panic!("Attack table index not empty. Error in Magics.");
                }
            }

            // No failures  during indexing. Store this magic.
            match piece {
                Piece::Rook => {
                    self.rook_magics[sq as usize] = magic;
                }
                Piece::Bishop => {
                    self.bishop_magics[sq as usize] = magic;
                }
                _ => panic!(
                    "can only generate magic for rook or bishop, {:?} was passed.",
                    piece
                ),
            }

            // Do the next magic.
            offset += permutations;
        }

        // All permutations (blocker boards) should have been indexed.
        let expectation = match piece {
            Piece::Rook => ROOK_TABLE_SIZE as u64,
            Piece::Bishop => BISHOP_TABLE_SIZE as u64,
            _ => panic!(
                "can only generate magic for rook or bishop, {:?} was passed.",
                piece
            ),
        };
        const ERROR: &str = "Initializing magics failed. Check magic numbers.";

        assert!(offset == expectation, "{}", ERROR);
    }
}
