use std::fmt;

#[allow(dead_code)]
pub fn str_to_u64(str: &str) -> u64 {
    u64::from_str_radix(str, 2).unwrap()
}

struct BinaryString { num: u64 }

impl BinaryString {
    pub fn new(num: u64) -> Self {
        BinaryString { num }
    }
}

impl fmt::Display for BinaryString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Binary::fmt(&self.num, f)
    }
}

struct VisualBinaryString { num: u64 }

#[allow(dead_code)]
impl VisualBinaryString {
    pub fn new(num: u64) -> Self {
        VisualBinaryString { num }
    }
}

impl fmt::Display for VisualBinaryString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bb = u64_to_str(self.num);
        let char_vec: Vec<char> = bb.chars().collect();

        write!(f,"
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

#[allow(dead_code)]
pub fn u64_to_str(num: u64) -> String {
    format!("{:0>64}", BinaryString::new(num))
}

#[allow(dead_code)]
pub fn print_bb(num: u64) {
    println!("{}", VisualBinaryString { num });
}
