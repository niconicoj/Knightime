use std::{
    fmt,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not, Shl, Shr},
};

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub struct Bitboard(u64);

impl Default for Bitboard {
    fn default() -> Self {
        Self(0x0)
    }
}

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for rank in (0u64..8u64).rev() {
            for file in 0u64..8u64 {
                if file == 0 {
                    print!("{}   ", rank + 1);
                }
                let square = (rank * 8 + file) as u8;
                match self.get_square(square) {
                    true => write!(f, "X ")?,
                    false => write!(f, "- ")?,
                }
            }
            println!();
        }

        writeln!(f)?;
        writeln!(f, "    a b c d e f g h")?;
        writeln!(f)?;
        writeln!(f, "  Bitboard : {:#018x}", self.0)?;
        writeln!(f)?;
        Ok(())
    }
}

impl Shl<u64> for Bitboard {
    type Output = Self;

    fn shl(self, rhs: u64) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl Shr<u64> for Bitboard {
    type Output = Self;

    fn shr(self, rhs: u64) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl Shl<u8> for Bitboard {
    type Output = Self;

    fn shl(self, rhs: u8) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl Shr<u8> for Bitboard {
    type Output = Self;

    fn shr(self, rhs: u8) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl BitAnd<u64> for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: u64) -> Self::Output {
        Self(self.0 & rhs)
    }
}

impl BitAnd<Bitboard> for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Bitboard) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign<u64> for Bitboard {
    fn bitand_assign(&mut self, rhs: u64) {
        self.0 &= rhs
    }
}

impl BitAndAssign<Bitboard> for Bitboard {
    fn bitand_assign(&mut self, rhs: Bitboard) {
        self.0 &= rhs.0
    }
}

impl BitOrAssign<Bitboard> for Bitboard {
    fn bitor_assign(&mut self, rhs: Bitboard) {
        self.0 |= rhs.0
    }
}

impl BitOrAssign<u64> for Bitboard {
    fn bitor_assign(&mut self, rhs: u64) {
        self.0 |= rhs
    }
}

impl BitOr<Bitboard> for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Bitboard) -> Self::Output {
        Bitboard::new(self.0 | rhs.0)
    }
}

impl PartialEq<u64> for Bitboard {
    fn eq(&self, other: &u64) -> bool {
        self.0 == *other
    }
}

impl Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl Bitboard {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn from_square(square: u8) -> Self {
        let mut bitboard = Self::default();
        bitboard.set_square(square);
        bitboard
    }

    pub fn get_square(&self, square: u8) -> bool {
        match self.0 & (1u64 << square) {
            0 => return false,
            _ => return true,
        }
    }

    pub fn set_square(&mut self, square: u8) {
        self.0 |= 1 << square;
    }

    pub fn clear_square(&mut self, square: u8) {
        self.0 &= !(1 << square);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::*;

    #[test]
    fn get_square_tests() {
        let bitboard = Bitboard::default();
        for index in 0..64 {
            assert_eq!(bitboard.get_square(index), false);
        }

        let bitboard = Bitboard::new(0x301);

        assert_eq!(bitboard.get_square(A1), true);
        assert_eq!(bitboard.get_square(A2), true);
        assert_eq!(bitboard.get_square(B2), true);
        assert_eq!(bitboard.get_square(C5), false);
    }

    #[test]
    fn set_square_tests() {
        let mut bitboard = Bitboard::default();
        bitboard.set_square(A1);
        assert_eq!(bitboard.get_square(A1), true);
        bitboard.set_square(B2);
        assert_eq!(bitboard.get_square(B2), true);
        bitboard.set_square(C1);
        bitboard.set_square(C3);
        assert_eq!(bitboard.0, 0x40205);
    }

    #[test]
    fn clear_square_tests() {
        let mut bitboard = Bitboard::default();
        bitboard.set_square(E1);
        bitboard.clear_square(E1);
        assert_eq!(bitboard.0, 0x0);
        bitboard.clear_square(E1);
        assert_eq!(bitboard.0, 0x0);
    }
}
