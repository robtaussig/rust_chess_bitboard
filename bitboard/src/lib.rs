use std::{
    fmt,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
};

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug, Default, Eq, Hash)]
pub struct BitBoard(pub u64);

impl BitAnd for BitBoard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitOr for BitBoard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitXor for BitBoard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for BitBoard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl Not for BitBoard {
    type Output = Self;

    fn not(self) -> Self::Output {
        BitBoard(!self.0)
    }
}

impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Binary::fmt(&self.0, f)
    }
}

impl BitBoard {
    pub fn to_str(&self) -> String {
        format!("{:0>64}", self)
    }

    pub fn from_str(str: &str) -> BitBoard {
        BitBoard(u64::from_str_radix(str, 2).unwrap())
    }

    pub fn print_bb(&self) {
        println!("{}", VisualBinaryString::new(self));
    }

    pub fn shl(&self, int: u8) -> BitBoard {
        BitBoard(self.0 << int)
    }

    pub fn shr(&self, int: u8) -> BitBoard {
        BitBoard(self.0 >> int)
    }

    pub fn bits(&self) -> BitIterator {
        BitIterator::new(self)
    }

    pub fn popcnt(&self) -> u32 {
        self.0.count_ones()
    }

    //Returns row index of first positive bit
    pub fn row(&self) -> usize {
        (self.0.leading_zeros() / 8) as usize
    }

    //Returns col index of first positive bit
    pub fn col(&self) -> usize {
        (7 - self.0.leading_zeros() % 8) as usize
    }
}

pub struct BitIterator {
    bb: u64,
}

impl BitIterator {
    pub fn new(bb: &BitBoard) -> Self {
        BitIterator { bb: bb.0 }
    }
}

impl Iterator for BitIterator {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        if self.bb != 0 {
            let bit = self.bb.trailing_zeros();
            self.bb &= self.bb - 1;
            Some(bit as usize)
        } else {
            None
        }
    }
}

struct VisualBinaryString {
    bb: u64,
}

impl VisualBinaryString {
    pub fn new(bb: &BitBoard) -> Self {
        VisualBinaryString { bb: bb.0 }
    }
}

impl fmt::Display for VisualBinaryString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let char_vec: Vec<char> = BitBoard(self.bb).to_str().chars().collect();

        write!(
            f,
            "
            {}{}{}{}{}{}{}{}
            {}{}{}{}{}{}{}{}
            {}{}{}{}{}{}{}{}
            {}{}{}{}{}{}{}{}
            {}{}{}{}{}{}{}{}
            {}{}{}{}{}{}{}{}
            {}{}{}{}{}{}{}{}
            {}{}{}{}{}{}{}{}
        ",
            char_vec[7],
            char_vec[6],
            char_vec[5],
            char_vec[4],
            char_vec[3],
            char_vec[2],
            char_vec[1],
            char_vec[0],
            char_vec[15],
            char_vec[14],
            char_vec[13],
            char_vec[12],
            char_vec[11],
            char_vec[10],
            char_vec[9],
            char_vec[8],
            char_vec[23],
            char_vec[22],
            char_vec[21],
            char_vec[20],
            char_vec[19],
            char_vec[18],
            char_vec[17],
            char_vec[16],
            char_vec[31],
            char_vec[30],
            char_vec[29],
            char_vec[28],
            char_vec[27],
            char_vec[26],
            char_vec[25],
            char_vec[24],
            char_vec[39],
            char_vec[38],
            char_vec[37],
            char_vec[36],
            char_vec[35],
            char_vec[34],
            char_vec[33],
            char_vec[32],
            char_vec[47],
            char_vec[46],
            char_vec[45],
            char_vec[44],
            char_vec[43],
            char_vec[42],
            char_vec[41],
            char_vec[40],
            char_vec[55],
            char_vec[54],
            char_vec[53],
            char_vec[52],
            char_vec[51],
            char_vec[50],
            char_vec[49],
            char_vec[48],
            char_vec[63],
            char_vec[62],
            char_vec[61],
            char_vec[60],
            char_vec[59],
            char_vec[58],
            char_vec[57],
            char_vec[56],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod bits {
        use super::*;

        #[test]
        fn it_works() {
            let mut total: u64 = 0;
            let bb = BitBoard(18374686479671623680);
            bb.bits().into_iter().for_each(|bit| {
                total += bit as u64;
            });
            assert_eq!(total, 476);
        }
    }
}
