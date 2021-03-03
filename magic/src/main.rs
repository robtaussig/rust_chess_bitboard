#![allow(dead_code)]
#![allow(unused_imports)]
mod between;
use between::*;
extern crate bitboard;
use crate::bitboard::*;

extern crate constants;
use crate::constants::*;

const OUT_DIR: &str = "gen";

fn main() {
    gen_between();

    let between_file = std::path::Path::new("").join("magic/src/between_generated.rs");
    let mut f = std::fs::File::create(between_file).unwrap();

    write_between(&mut f);
}
