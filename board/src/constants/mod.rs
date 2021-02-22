#[path = "../util/mod.rs"]
mod util;

pub const INITIAL_WHITE_PAWNS: u64 = 65280;
pub const INITIAL_WHITE_KNIGHTS: u64 = 66;
pub const INITIAL_WHITE_BISHOPS: u64 = 36;
pub const INITIAL_WHITE_ROOKS: u64 = 129;
pub const INITIAL_WHITE_QUEENS: u64 = 8;
pub const INITIAL_WHITE_KINGS: u64 = 16;
pub const INITIAL_BLACK_PAWNS: u64 = 71776119061217280;
pub const INITIAL_BLACK_KNIGHTS: u64 = 4755801206503243776;
pub const INITIAL_BLACK_BISHOPS: u64 = 2594073385365405696;
pub const INITIAL_BLACK_ROOKS: u64 = 9295429630892703744;
pub const INITIAL_BLACK_QUEENS: u64 = 576460752303423488;
pub const INITIAL_BLACK_KINGS: u64 = 1152921504606846976;

pub const RANK_1: u64 = 255;
pub const RANK_2: u64 = 65280;
pub const RANK_3: u64 = 16711680;
pub const RANK_4: u64 = 4278190080;
pub const RANK_5: u64 = 1095216660480;
pub const RANK_6: u64 = 280375465082880;
pub const RANK_7: u64 = 71776119061217280;
pub const RANK_8: u64 = 18374686479671623680;

#[allow(dead_code)]
pub const CLEAR_RANK_1: u64 = !RANK_1;
#[allow(dead_code)]
pub const CLEAR_RANK_2: u64 = !RANK_2;
#[allow(dead_code)]
pub const CLEAR_RANK_3: u64 = !RANK_3;
#[allow(dead_code)]
pub const CLEAR_RANK_4: u64 = !RANK_4;
#[allow(dead_code)]
pub const CLEAR_RANK_5: u64 = !RANK_5;
#[allow(dead_code)]
pub const CLEAR_RANK_6: u64 = !RANK_6;
#[allow(dead_code)]
pub const CLEAR_RANK_7: u64 = !RANK_7;
#[allow(dead_code)]
pub const CLEAR_RANK_8: u64 = !RANK_8;

#[allow(dead_code)]
pub const RANK_MASK: [u64; 8] = [
    RANK_8,
    RANK_7,
    RANK_6,
    RANK_5,
    RANK_4,
    RANK_3,
    RANK_2,
    RANK_1,
];

pub const A_FILE: u64 = 72340172838076673;
pub const B_FILE: u64 = 144680345676153346;
pub const C_FILE: u64 = 289360691352306692;
pub const D_FILE: u64 = 578721382704613384;
pub const E_FILE: u64 = 1157442765409226768;
pub const F_FILE: u64 = 2314885530818453536;
pub const G_FILE: u64 = 4629771061636907072;
pub const H_FILE: u64 = 9259542123273814144;

#[allow(dead_code)]
pub const CLEAR_A_FILE: u64 = !A_FILE;
#[allow(dead_code)]
pub const CLEAR_B_FILE: u64 = !B_FILE;
#[allow(dead_code)]
pub const CLEAR_C_FILE: u64 = !C_FILE;
#[allow(dead_code)]
pub const CLEAR_D_FILE: u64 = !D_FILE;
#[allow(dead_code)]
pub const CLEAR_E_FILE: u64 = !E_FILE;
#[allow(dead_code)]
pub const CLEAR_F_FILE: u64 = !F_FILE;
#[allow(dead_code)]
pub const CLEAR_G_FILE: u64 = !G_FILE;
#[allow(dead_code)]
pub const CLEAR_H_FILE: u64 = !H_FILE;

#[allow(dead_code)]
pub const FILE_MASK: [u64; 8] = [
    A_FILE,
    B_FILE,
    C_FILE,
    D_FILE,
    E_FILE,
    F_FILE,
    G_FILE,
    H_FILE,
];

pub const A8_SQUARE: u64 = A_FILE & RANK_8;
pub const B8_SQUARE: u64 = B_FILE & RANK_8;
pub const C8_SQUARE: u64 = C_FILE & RANK_8;
pub const D8_SQUARE: u64 = D_FILE & RANK_8;
pub const E8_SQUARE: u64 = E_FILE & RANK_8;
pub const F8_SQUARE: u64 = F_FILE & RANK_8;
pub const G8_SQUARE: u64 = G_FILE & RANK_8;
pub const H8_SQUARE: u64 = H_FILE & RANK_8;
pub const A7_SQUARE: u64 = A_FILE & RANK_7;
pub const B7_SQUARE: u64 = B_FILE & RANK_7;
pub const C7_SQUARE: u64 = C_FILE & RANK_7;
pub const D7_SQUARE: u64 = D_FILE & RANK_7;
pub const E7_SQUARE: u64 = E_FILE & RANK_7;
pub const F7_SQUARE: u64 = F_FILE & RANK_7;
pub const G7_SQUARE: u64 = G_FILE & RANK_7;
pub const H7_SQUARE: u64 = H_FILE & RANK_7;
pub const A6_SQUARE: u64 = A_FILE & RANK_6;
pub const B6_SQUARE: u64 = B_FILE & RANK_6;
pub const C6_SQUARE: u64 = C_FILE & RANK_6;
pub const D6_SQUARE: u64 = D_FILE & RANK_6;
pub const E6_SQUARE: u64 = E_FILE & RANK_6;
pub const F6_SQUARE: u64 = F_FILE & RANK_6;
pub const G6_SQUARE: u64 = G_FILE & RANK_6;
pub const H6_SQUARE: u64 = H_FILE & RANK_6;
pub const A5_SQUARE: u64 = A_FILE & RANK_5;
pub const B5_SQUARE: u64 = B_FILE & RANK_5;
pub const C5_SQUARE: u64 = C_FILE & RANK_5;
pub const D5_SQUARE: u64 = D_FILE & RANK_5;
pub const E5_SQUARE: u64 = E_FILE & RANK_5;
pub const F5_SQUARE: u64 = F_FILE & RANK_5;
pub const G5_SQUARE: u64 = G_FILE & RANK_5;
pub const H5_SQUARE: u64 = H_FILE & RANK_5;
pub const A4_SQUARE: u64 = A_FILE & RANK_4;
pub const B4_SQUARE: u64 = B_FILE & RANK_4;
pub const C4_SQUARE: u64 = C_FILE & RANK_4;
pub const D4_SQUARE: u64 = D_FILE & RANK_4;
pub const E4_SQUARE: u64 = E_FILE & RANK_4;
pub const F4_SQUARE: u64 = F_FILE & RANK_4;
pub const G4_SQUARE: u64 = G_FILE & RANK_4;
pub const H4_SQUARE: u64 = H_FILE & RANK_4;
pub const A3_SQUARE: u64 = A_FILE & RANK_3;
pub const B3_SQUARE: u64 = B_FILE & RANK_3;
pub const C3_SQUARE: u64 = C_FILE & RANK_3;
pub const D3_SQUARE: u64 = D_FILE & RANK_3;
pub const E3_SQUARE: u64 = E_FILE & RANK_3;
pub const F3_SQUARE: u64 = F_FILE & RANK_3;
pub const G3_SQUARE: u64 = G_FILE & RANK_3;
pub const H3_SQUARE: u64 = H_FILE & RANK_3;
pub const A2_SQUARE: u64 = A_FILE & RANK_2;
pub const B2_SQUARE: u64 = B_FILE & RANK_2;
pub const C2_SQUARE: u64 = C_FILE & RANK_2;
pub const D2_SQUARE: u64 = D_FILE & RANK_2;
pub const E2_SQUARE: u64 = E_FILE & RANK_2;
pub const F2_SQUARE: u64 = F_FILE & RANK_2;
pub const G2_SQUARE: u64 = G_FILE & RANK_2;
pub const H2_SQUARE: u64 = H_FILE & RANK_2;
pub const A1_SQUARE: u64 = A_FILE & RANK_1;
pub const B1_SQUARE: u64 = B_FILE & RANK_1;
pub const C1_SQUARE: u64 = C_FILE & RANK_1;
pub const D1_SQUARE: u64 = D_FILE & RANK_1;
pub const E1_SQUARE: u64 = E_FILE & RANK_1;
pub const F1_SQUARE: u64 = F_FILE & RANK_1;
pub const G1_SQUARE: u64 = G_FILE & RANK_1;
pub const H1_SQUARE: u64 = H_FILE & RANK_1;

pub const SQUARE_MASK: [u64; 64] = [
    A8_SQUARE,
    B8_SQUARE,
    C8_SQUARE,
    D8_SQUARE,
    E8_SQUARE,
    F8_SQUARE,
    G8_SQUARE,
    H8_SQUARE,
    A7_SQUARE,
    B7_SQUARE,
    C7_SQUARE,
    D7_SQUARE,
    E7_SQUARE,
    F7_SQUARE,
    G7_SQUARE,
    H7_SQUARE,
    A6_SQUARE,
    B6_SQUARE,
    C6_SQUARE,
    D6_SQUARE,
    E6_SQUARE,
    F6_SQUARE,
    G6_SQUARE,
    H6_SQUARE,
    A5_SQUARE,
    B5_SQUARE,
    C5_SQUARE,
    D5_SQUARE,
    E5_SQUARE,
    F5_SQUARE,
    G5_SQUARE,
    H5_SQUARE,
    A4_SQUARE,
    B4_SQUARE,
    C4_SQUARE,
    D4_SQUARE,
    E4_SQUARE,
    F4_SQUARE,
    G4_SQUARE,
    H4_SQUARE,
    A3_SQUARE,
    B3_SQUARE,
    C3_SQUARE,
    D3_SQUARE,
    E3_SQUARE,
    F3_SQUARE,
    G3_SQUARE,
    H3_SQUARE,
    A2_SQUARE,
    B2_SQUARE,
    C2_SQUARE,
    D2_SQUARE,
    E2_SQUARE,
    F2_SQUARE,
    G2_SQUARE,
    H2_SQUARE,
    A1_SQUARE,
    B1_SQUARE,
    C1_SQUARE,
    D1_SQUARE,
    E1_SQUARE,
    F1_SQUARE,
    G1_SQUARE,
    H1_SQUARE,
];

#[cfg(test)]
mod tests {
    
    use util::{str_to_u64};
    use super::*;

    #[test]
    fn ranks_work() {
        assert_eq!(
            str_to_u64("1111111100000000000000000000000000000000000000000000000000000000"),
            RANK_8,
        );

        assert_eq!(
            str_to_u64("0000000011111111000000000000000000000000000000000000000000000000"),
            RANK_7,
        );

        assert_eq!(
            str_to_u64("0000000000000000111111110000000000000000000000000000000000000000"),
            RANK_6,
        );

        assert_eq!(
            str_to_u64("0000000000000000000000001111111100000000000000000000000000000000"),
            RANK_5,
        );

        assert_eq!(
            str_to_u64("0000000000000000000000000000000011111111000000000000000000000000"),
            RANK_4,
        );

        assert_eq!(
            str_to_u64("0000000000000000000000000000000000000000111111110000000000000000"),
            RANK_3,
        );

        assert_eq!(
            str_to_u64("0000000000000000000000000000000000000000000000001111111100000000"),
            RANK_2,
        );

        assert_eq!(
            str_to_u64("0000000000000000000000000000000000000000000000000000000011111111"),
            RANK_1,
        );
    }

    #[test]
    fn files_work() {
        assert_eq!(
            str_to_u64("1000000010000000100000001000000010000000100000001000000010000000"),
            H_FILE,
        );

        assert_eq!(
            str_to_u64("0100000001000000010000000100000001000000010000000100000001000000"),
            G_FILE,
        );

        assert_eq!(
            str_to_u64("0010000000100000001000000010000000100000001000000010000000100000"),
            F_FILE,
        );

        assert_eq!(
            str_to_u64("0001000000010000000100000001000000010000000100000001000000010000"),
            E_FILE,
        );

        assert_eq!(
            str_to_u64("0000100000001000000010000000100000001000000010000000100000001000"),
            D_FILE,
        );

        assert_eq!(
            str_to_u64("0000010000000100000001000000010000000100000001000000010000000100"),
            C_FILE,
        );

        assert_eq!(
            str_to_u64("0000001000000010000000100000001000000010000000100000001000000010"),
            B_FILE,
        );

        assert_eq!(
            str_to_u64("0000000100000001000000010000000100000001000000010000000100000001"),
            A_FILE,
        );
    }

    #[test]
    fn squares_work() {
        assert_eq!(
            str_to_u64("1000000000000000000000000000000000000000000000000000000000000000"),
            H8_SQUARE,
        );
        assert_eq!(
            str_to_u64("0100000000000000000000000000000000000000000000000000000000000000"),
            G8_SQUARE,
        );
        assert_eq!(
            str_to_u64("0010000000000000000000000000000000000000000000000000000000000000"),
            F8_SQUARE,
        );
        assert_eq!(
            str_to_u64("0001000000000000000000000000000000000000000000000000000000000000"),
            E8_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000100000000000000000000000000000000000000000000000000000000000"),
            D8_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000010000000000000000000000000000000000000000000000000000000000"),
            C8_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000001000000000000000000000000000000000000000000000000000000000"),
            B8_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000100000000000000000000000000000000000000000000000000000000"),
            A8_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000010000000000000000000000000000000000000000000000000000000"),
            H7_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000001000000000000000000000000000000000000000000000000000000"),
            G7_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000100000000000000000000000000000000000000000000000000000"),
            F7_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000010000000000000000000000000000000000000000000000000000"),
            E7_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000001000000000000000000000000000000000000000000000000000"),
            D7_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000100000000000000000000000000000000000000000000000000"),
            C7_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000010000000000000000000000000000000000000000000000000"),
            B7_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000001000000000000000000000000000000000000000000000000"),
            A7_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000000100000000000000000000000000000000000000000000000"),
            H6_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000000010000000000000000000000000000000000000000000000"),
            G6_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000000001000000000000000000000000000000000000000000000"),
            F6_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000000000100000000000000000000000000000000000000000000"),
            E6_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000000000010000000000000000000000000000000000000000000"),
            D6_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000000000001000000000000000000000000000000000000000000"),
            C6_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000000000000100000000000000000000000000000000000000000"),
            B6_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000000000000010000000000000000000000000000000000000000"),
            A6_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000000000000001000000000000000000000000000000000000000"),
            H5_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000000000000000100000000000000000000000000000000000000"),
            G5_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000000000000000010000000000000000000000000000000000000"),
            F5_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000000000000000001000000000000000000000000000000000000"),
            E5_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000000000000000000100000000000000000000000000000000000"),
            D5_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000000000000000000010000000000000000000000000000000000"),
            C5_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000000000000000000001000000000000000000000000000000000"),
            B5_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000000000000000000000100000000000000000000000000000000"),
            A5_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000000000000000000000010000000000000000000000000000000"),
            H4_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000000000000000000000001000000000000000000000000000000"),
            G4_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000000000000000000000000100000000000000000000000000000"),
            F4_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000000000000000000000000010000000000000000000000000000"),
            E4_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000000000000000000000000001000000000000000000000000000"),
            D4_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000000000000000000000000000100000000000000000000000000"),
            C4_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000000000000000000000000000010000000000000000000000000"),
            B4_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000000000000000000000000000001000000000000000000000000"),
            A4_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000000000000000000000000000000100000000000000000000000"),
            H3_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000000000000000000000000000000010000000000000000000000"),
            G3_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000000000000000000000000000000001000000000000000000000"),
            F3_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000000000000000000000000000000000100000000000000000000"),
            E3_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000000000000000000000000000000000010000000000000000000"),
            D3_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000000000000000000000000000000000001000000000000000000"),
            C3_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000000000000000000000000000000000000100000000000000000"),
            B3_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000000000000000000000000000000000000010000000000000000"),
            A3_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000000000000000000000000000000000000001000000000000000"),
            H2_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000000000000000000000000000000000000000100000000000000"),
            G2_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000000000000000000000000000000000000000010000000000000"),
            F2_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000000000000000000000000000000000000000001000000000000"),
            E2_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000000000000000000000000000000000000000000100000000000"),
            D2_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000000000000000000000000000000000000000000010000000000"),
            C2_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000000000000000000000000000000000000000000001000000000"),
            B2_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000000000000000000000000000000000000000000000100000000"),
            A2_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000000000000000000000000000000000000000000000010000000"),
            H1_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000000000000000000000000000000000000000000000001000000"),
            G1_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000000000000000000000000000000000000000000000000100000"),
            F1_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000000000000000000000000000000000000000000000000010000"),
            E1_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000000000000000000000000000000000000000000000000001000"),
            D1_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000000000000000000000000000000000000000000000000000100"),
            C1_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000000000000000000000000000000000000000000000000000010"),
            B1_SQUARE,
        );
        assert_eq!(
            str_to_u64("0000000000000000000000000000000000000000000000000000000000000001"),
            A1_SQUARE,
        );
    }
}
