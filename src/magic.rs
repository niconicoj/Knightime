use crate::{
    bitboard::Bitboard,
    constants::SQUARE_NAME,
    defs::{Piece, Square},
    move_generator::MoveGenerator,
};

use rand::{Rng, SeedableRng};
use rand_chacha::ChaChaRng;

pub const ROOK_MAGIC_NUMBERS: [u64; 64] = [
    0x0480102240008000,
    0x00400020021008c0,
    0x2080200210006880,
    0x0180048030020800,
    0x8200081020148600,
    0xa100010042140088,
    0x0200088200080403,
    0x1a80090000a04080,
    0x0006800880400228,
    0x0400401000a00040,
    0x6198808010002000,
    0x8006002821401200,
    0x0601000800910014,
    0x5200800400801200,
    0xc401010100040200,
    0x0960801340800900,
    0x8010828001c002a0,
    0x1000424010002000,
    0x0109010020004014,
    0x0000808018001002,
    0x1028018024000880,
    0x0008480114204010,
    0x0502910100040200,
    0x0020020000804104,
    0x0000812580044008,
    0x4010024040002000,
    0x2020018280100020,
    0x2060900100201900,
    0x0480050100280010,
    0x84aa0002000c1088,
    0x4882080c00010210,
    0x0002800880004100,
    0x0000304000800080,
    0x00a0005002400460,
    0x8102410031002000,
    0x0000201001000901,
    0x0000804400800801,
    0x0030800400804200,
    0x1000080284000530,
    0x28040400820010c1,
    0x0106800240008020,
    0x4140002002868040,
    0x0011002004110040,
    0x0248003000808008,
    0x8211060016120020,
    0x8002008004008100,
    0x04000230880c0003,
    0x2210104084020001,
    0x004024d502018200,
    0x9110012002401540,
    0x0038200210088080,
    0x8028809001380080,
    0x1043010800045100,
    0x80a08400800a0080,
    0x0010220910181c00,
    0x4281002082420100,
    0x2420800010210141,
    0x9280442182010412,
    0x800080614201102a,
    0x2008200410010049,
    0x0002010c60081046,
    0x008200810804100a,
    0x4202000401183182,
    0x8102004824018102,
];

pub const BISHOP_MAGIC_NUMBERS: [u64; 64] = [
    0x00080284840c0080,
    0x400812041400f240,
    0x0070018087000080,
    0x0110908600201800,
    0x0188484004020049,
    0x0102021004004064,
    0x00e4042104124480,
    0xc00040c804104212,
    0x002040a4a1440110,
    0x20840d0808004180,
    0x1248081089020000,
    0x8001644100228000,
    0x4000020211100040,
    0x4200120a11040209,
    0x0000008090082000,
    0x0000010188b00800,
    0x0007002094100200,
    0x0064002081420202,
    0x0004000818240210,
    0x01c8000882044038,
    0x9082200400a00840,
    0x0001002201010100,
    0x0404004104880472,
    0x0001000204808481,
    0x0120080022080500,
    0x10261014a0810a02,
    0x00024050a8020041,
    0x0004180003005100,
    0x8485010080904000,
    0x419129000a014104,
    0x40284040a1140603,
    0xc1810a000c209400,
    0x080446400208100c,
    0x000c100800020200,
    0x4802011000010240,
    0x4000220080080080,
    0x0408410040040040,
    0x8214080202806084,
    0x3010640080004240,
    0x0818248420808200,
    0x10a8041004088808,
    0x0002441004000902,
    0x9014220222041001,
    0x180011e018004100,
    0x1c44042502140400,
    0x0440280881019021,
    0x0018480b044c1c00,
    0x1010130200a00085,
    0x2009011002203000,
    0x0005010801040094,
    0xc30212010088088c,
    0x0101080a8c0c0400,
    0x0a0000881b040000,
    0x0104302041610b00,
    0x00a0208102108080,
    0x0404080819042000,
    0x0000424c00a03000,
    0x0410204220900810,
    0x220004a202011440,
    0x4400060020208840,
    0x4808000010212600,
    0x0802002002820a00,
    0x0823242104040080,
    0x0220411041010060,
];

pub const ROOK_TABLE_SIZE: usize = 102_400; // Total permutations of all rook blocker boards.
pub const BISHOP_TABLE_SIZE: usize = 5_248; // Total permutations of all bishop blocker boards.

pub fn find_magics(piece: Piece) {
    // Create working variables.
    let is_rook = match piece {
        Piece::Rook => true,
        _ => false,
    };
    let mut table = match piece {
        Piece::Rook => vec![Bitboard::default(); ROOK_TABLE_SIZE],
        Piece::Bishop => vec![Bitboard::default(); BISHOP_TABLE_SIZE],
        _ => panic!(
            "can only generate magic for rook or bishop, {:?} was passed.",
            piece
        ),
    };
    let mut random = ChaChaRng::from_entropy();
    let mut offset = 0;

    match piece {
        Piece::Rook => {
            println!("Finding magics for rook");
        }
        Piece::Bishop => {
            println!("Finding magics for bishop");
        }
        _ => panic!(
            "can only generate magic for rook or bishop, {:?} was passed.",
            piece
        ),
    }
    for sq in 0..64 {
        // Create the mask for either the rook or bishop.
        let mask = match piece {
            Piece::Rook => MoveGenerator::mask_rook_attacks(sq),
            Piece::Bishop => MoveGenerator::mask_bishop_attacks(sq),
            _ => panic!(
                "can only generate magic for rook or bishop, {:?} was passed.",
                piece
            ),
        };

        // Precalculate needed values.
        let bits = mask.get_value().count_ones(); // Number of set bits in the mask
        let permutations = 2u64.pow(bits); // Number of blocker boards to be indexed.
        let end = offset + permutations - 1; // End index in the attack table.

        // Create blocker boards for the current mask.
        let blocker_boards = mask.get_blocker_boards();

        // Create attack boards for the current square/blocker combo (either
        // rook or bishop).
        let attack_boards = match piece {
            Piece::Rook => MoveGenerator::generate_rook_attack_boards(sq, &blocker_boards),
            Piece::Bishop => MoveGenerator::generate_bishop_attack_boards(sq, &blocker_boards),
            _ => panic!(
                "can only generate magic for rook or bishop, {:?} was passed.",
                piece
            ),
        };

        // Done calculating needed data. Create a new magic.
        let mut try_this: Magic = Default::default(); // New magic
        let mut found = false; // While loop breaker if magic works;
        let mut attempts = 0; // Track needed attempts to find the magic.

        // Set up the new magic with the values we already know.
        try_this.mask = mask;
        try_this.shift = (64 - bits) as u8;
        try_this.offset = offset;

        // Start finding a magic that works for this square, for all permuations.
        while !found {
            attempts += 1; // Next attempt to find magic.
            found = true; // Assume this new magic will work.

            // Create a random magic number to test.
            try_this.nr = random.gen::<u64>() & random.gen::<u64>() & random.gen::<u64>();

            // Now try all possible permutations of blocker boards on this square.
            for i in 0..permutations {
                // Get the index where the magic for this blocker board
                // needs to go (if it works.)
                let next = i as usize;
                let index = try_this.get_index(blocker_boards[next]);

                if table[index] == 0 {
                    // Check if we're within the expected range
                    let fail_low = index < offset as usize;
                    let fail_high = index > end as usize;
                    assert!(!fail_low && !fail_high, "Indexing error.");

                    // We found a working magic.
                    table[index] = attack_boards[next];
                } else {
                    // The table at this index is not empty. We have a
                    // collision. This magic doesn't work. Wipe the part of
                    // the table we are working with. Try a new number.
                    for wipe_index in offset..=end {
                        table[wipe_index as usize] = Bitboard::default();
                    }
                    found = false;
                    break;
                }
            }
        }

        // We got out of the loop and found a random magic number that can
        // index all the attack boards for a rook/bishop for a single
        // square without a collision. Report this number.
        found_magic(sq, try_this, offset, end, attempts);

        // Set table offset for next magic.
        offset += permutations;
    }

    // Check if the entire table is correct. The offset should be equal to
    // the size of the table. If it isn't, we skipped permuations and thus
    // have some sort of error in our code above.
    let r_ts = ROOK_TABLE_SIZE as u64;
    let b_ts = BISHOP_TABLE_SIZE as u64;
    let expected = if is_rook { r_ts } else { b_ts };
    const ERROR: &str = "Creating magics failed. Permutations were skipped.";

    assert!(offset == expected, "{}", ERROR);
}

fn found_magic(square: Square, m: Magic, offset: u64, end: u64, attempts: u64) {
    println!(
        "{}: {:24}u64 (offset: {:6}, end: {:6}, attempts: {})",
        SQUARE_NAME[square as usize], m.nr, offset, end, attempts
    );
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Magic {
    pub mask: Bitboard,
    pub shift: u8,
    pub offset: u64,
    pub nr: u64,
}

impl Default for Magic {
    fn default() -> Self {
        Self {
            mask: Bitboard::default(),
            shift: 0,
            offset: 0,
            nr: 0,
        }
    }
}

impl Magic {
    pub fn get_index(&self, occupancy: Bitboard) -> usize {
        let blockerboard = occupancy & self.mask;
        ((blockerboard.get_value().wrapping_mul(self.nr) >> self.shift) + self.offset) as usize
    }
}
