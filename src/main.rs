use std::fmt;

fn main() {
    let y = available_moves(START_POSITION, true);
    let x : u64 = C5|D6|F4|E3;
    println!("Available:\n\n{}\n\nActual:\n\n{}\n\n", Othello{white:y,black:0x0}, Othello{white:x,black:0x0});
}


const START_POSITION : Othello = Othello{white: E5|D4, black:E4|D5};

const A1 : u64 = 0x1;
const B1 : u64 = 0x2;
const C1 : u64 = 0x4;
const D1 : u64 = 0x8;
const E1 : u64 = 0x10;
const F1 : u64 = 0x20;
const G1 : u64 = 0x40;
const H1 : u64 = 0x80;
const A2 : u64 = 0x100;
const B2 : u64 = 0x200;
const C2 : u64 = 0x400;
const D2 : u64 = 0x800;
const E2 : u64 = 0x1000;
const F2 : u64 = 0x2000;
const G2 : u64 = 0x4000;
const H2 : u64 = 0x8000;
const A3 : u64 = 0x10000;
const B3 : u64 = 0x20000;
const C3 : u64 = 0x40000;
const D3 : u64 = 0x80000;
const E3 : u64 = 0x100000;
const F3 : u64 = 0x200000;
const G3 : u64 = 0x400000;
const H3 : u64 = 0x800000;
const A4 : u64 = 0x1000000;
const B4 : u64 = 0x2000000;
const C4 : u64 = 0x4000000;
const D4 : u64 = 0x8000000;
const E4 : u64 = 0x10000000;
const F4 : u64 = 0x20000000;
const G4 : u64 = 0x40000000;
const H4 : u64 = 0x80000000;
const A5 : u64 = 0x100000000;
const B5 : u64 = 0x200000000;
const C5 : u64 = 0x400000000;
const D5 : u64 = 0x800000000;
const E5 : u64 = 0x1000000000;
const F5 : u64 = 0x2000000000;
const G5 : u64 = 0x4000000000;
const H5 : u64 = 0x8000000000;
const A6 : u64 = 0x10000000000;
const B6 : u64 = 0x20000000000;
const C6 : u64 = 0x40000000000;
const D6 : u64 = 0x80000000000;
const E6 : u64 = 0x100000000000;
const F6 : u64 = 0x200000000000;
const G6 : u64 = 0x400000000000;
const H6 : u64 = 0x800000000000;
const A7 : u64 = 0x1000000000000;
const B7 : u64 = 0x2000000000000;
const C7 : u64 = 0x4000000000000;
const D7 : u64 = 0x8000000000000;
const E7 : u64 = 0x10000000000000;
const F7 : u64 = 0x20000000000000;
const G7 : u64 = 0x40000000000000;
const H7 : u64 = 0x80000000000000;
const A8 : u64 = 0x100000000000000;
const B8 : u64 = 0x200000000000000;
const C8 : u64 = 0x400000000000000;
const D8 : u64 = 0x800000000000000;
const E8 : u64 = 0x1000000000000000;
const F8 : u64 = 0x2000000000000000;
const G8 : u64 = 0x4000000000000000;
const H8 : u64 = 0x8000000000000000;

const FILE_A : u64 = A1|A2|A3|A4|A5|A6|A7|A8;
const FILE_B : u64 = B1|B2|B3|B4|B5|B6|B7|B8;
const FILE_C : u64 = C1|C2|C3|C4|C5|C6|C7|C8;
const FILE_D : u64 = D1|D2|D3|D4|D5|D6|D7|D8;
const FILE_E : u64 = E1|E2|E3|E4|E5|E6|E7|E8;
const FILE_F : u64 = F1|F2|F3|F4|F5|F6|F7|F8;
const FILE_G : u64 = G1|G2|G3|G4|G5|G6|G7|G8;
const FILE_H : u64 = H1|H2|H3|H4|H5|H6|H7|H8;

const RANK_1 : u64 = A1|B1|C1|D1|E1|F1|G1|H1;
const RANK_2 : u64 = A2|B2|C2|D2|E2|F2|G2|H2;
const RANK_3 : u64 = A3|B3|C3|D3|E3|F3|G3|H3;
const RANK_4 : u64 = A4|B4|C4|D4|E4|F4|G4|H4;
const RANK_5 : u64 = A5|B5|C5|D5|E5|F5|G5|H5;
const RANK_6 : u64 = A6|B6|C6|D6|E6|F6|G6|H6;
const RANK_7 : u64 = A7|B7|C7|D7|E7|F7|G7|H7;
const RANK_8 : u64 = A8|B8|C8|D8|E8|F8|G8|H8;

const A1_H8 : u64 = A1|B2|C3|D4|E5|F6|G7|H8;
const A2_G8 : u64 = A2|B3|C4|D5|E6|F7|G8;
const B1_H7 : u64 = B1|C2|D3|E4|F5|G6|H7;
const C1_H6 : u64 = C1|D2|E3|F4|G5|H6;
const A3_F8 : u64 = A3|B4|C5|D6|E7|F8;
const A4_E8 : u64 = A4|B5|C6|D7|E8;
const D1_H5 : u64 = D1|E2|F3|G4|H5;
const A5_D8 : u64 = A5|B6|C7|D8;
const E1_H4 : u64 = E1|F2|G3|H4;
const A6_C8 : u64 = A6|B7|C8;
const F1_H3 : u64 = F1|G2|H3;
const A7_B8 : u64 = A7|B8;
const G1_H2 : u64 = G1|H2;

const A8_H1 : u64 = A8|B7|C6|D5|E4|F3|G2|H1;
const A7_G1 : u64 = A7|B6|C5|D4|E3|F2|G1;
const B8_H2 : u64 = B8|C7|D6|E5|F4|G3|H2;
const C8_H3 : u64 = C8|D7|E6|F5|G4|H3;
const A6_F1 : u64 = A6|B5|C4|D3|E2|F1;
const A5_E1 : u64 = A5|B4|C3|D2|E1;
const D8_H4 : u64 = D8|E7|F6|G5|H4;
const A4_D1 : u64 = A4|B3|C2|D1;
const E8_H5 : u64 = E8|F7|G6|H5;
const A3_C1 : u64 = A3|B2|C1;
const F8_H6 : u64 = F8|G7|H6;
const A2_B1 : u64 = A2|B1;
const G8_H7 : u64 = G8|H7;

//the bit mask for the tiles flipped by the board
const FLIP_MASK : [u64; 64] = 
[
    FILE_A|RANK_1|A1_H8,        FILE_B|RANK_1|B1_H7|A2_B1,  FILE_C|RANK_1|A3_C1|C1_H6,  FILE_D|RANK_1|A4_D1|D1_H5,
    FILE_E|RANK_1|A5_E1|E1_H4,  FILE_F|RANK_1|F1_H3|A6_F1,  FILE_G|RANK_1|G1_H2|A7_G1,  FILE_H|RANK_1|A8_H1,
    
    FILE_A|RANK_2|A2_G8|A2_B1,  FILE_B|RANK_2|A1_H8|A3_C1,  FILE_C|RANK_2|A4_D1|B1_H7,  FILE_D|RANK_2|C1_H6|A5_E1,
    FILE_E|RANK_2|A6_F1|F1_H3,  FILE_F|RANK_2|A7_G1|E1_H4,  FILE_G|RANK_2|A8_H1|F1_H3,  FILE_H|RANK_2|G1_H2|B8_H2,
    
    FILE_A|RANK_3|A3_C1|A3_F8,  FILE_B|RANK_3|A2_G8|A4_D1,  FILE_C|RANK_3|A5_E1|A1_H8,  FILE_D|RANK_3|A6_F1|B1_H7,
    FILE_E|RANK_3|A7_G1|C1_H6,  FILE_F|RANK_3|A8_H1|D1_H5,  FILE_G|RANK_3|B8_H2|E1_H4,  FILE_H|RANK_3|F1_H3|C8_H3,

    FILE_A|RANK_4|A4_D1|A4_E8,  FILE_B|RANK_4|A5_E1|A3_F8,  FILE_C|RANK_4|A2_G8|A6_F1,  FILE_D|RANK_4|A7_G1|A1_H8,
    FILE_E|RANK_4|A8_H1|B1_H7,  FILE_F|RANK_4|B8_H2|C1_H6,  FILE_G|RANK_4|C8_H3|D1_H5,  FILE_H|RANK_4|E1_H4|D8_H4,

    FILE_A|RANK_5|A5_D8|A5_E1,  FILE_B|RANK_5|A6_F1|A4_E8,  FILE_C|RANK_5|A7_G1|A3_F8,  FILE_D|RANK_5|A2_G8|A8_H1,
    FILE_E|RANK_5|A1_H8|B8_H2,  FILE_F|RANK_5|C8_H3|B1_H7,  FILE_G|RANK_5|C1_H6|D8_H4,  FILE_H|RANK_5|D1_H5|E8_H5,

    FILE_A|RANK_6|A6_C8|A6_F1,  FILE_B|RANK_6|A7_G1|A5_D8,  FILE_C|RANK_6|A4_E8|A8_H1,  FILE_D|RANK_6|B8_H2|A3_F8,
    FILE_E|RANK_6|C8_H3|A2_G8,  FILE_F|RANK_6|A1_H8|D8_H4,  FILE_G|RANK_6|E8_H5|B1_H7,  FILE_H|RANK_6|F8_H6|C1_H6,

    FILE_A|RANK_7|A7_B8|A7_G1,  FILE_B|RANK_7|A8_H1|A6_C8,  FILE_C|RANK_7|A5_D8|B8_H2,  FILE_D|RANK_7|C8_H3|A4_E8,
    FILE_E|RANK_7|A3_F8|D8_H4,  FILE_F|RANK_7|A2_G8|E8_H5,  FILE_G|RANK_7|F8_H6|A1_H8,  FILE_H|RANK_7|G8_H7|B1_H7,

    FILE_A|RANK_8|A8_H1,        FILE_B|RANK_8|A7_B8|B8_H2,  FILE_C|RANK_8|A6_C8|C8_H3,  FILE_D|RANK_8|A5_D8|D8_H4,
    FILE_E|RANK_8|A4_E8|E8_H5,  FILE_F|RANK_8|A3_F8|F8_H6,  FILE_G|RANK_8|A2_G8|G8_H7,  FILE_H|RANK_8|A1_H8
];


struct Othello {
    white : u64,
    black : u64,
}


impl fmt::Display for Othello {
    fn fmt(&self, f: &mut fmt::Formatter)->fmt::Result {
        assert_eq!(self.white & self.black, 0);
        let mut s = String::with_capacity(71);
        for i in 0..64 {
            if (i % 8)==0 && i != 0 {
                s.push('\n');
            }
            let x : u64 = 1<<i;
            if (self.white & x) != 0 {
                s.push('O');
            } else if (self.black & x) != 0 {
                s.push('X');
            } else {
                s.push('-');
            }
        }
        return write!(f, "{}", s);
    }
}


fn debruins(mut board : u64)->usize {
    const TABLE : [usize; 64] = [63, 30,  3, 32, 25, 41, 22, 33,
                               15, 50, 42, 13, 11, 53, 19, 34,
                               61, 29,  2, 51, 21, 43, 45, 10,
                               18, 47,  1, 54,  9, 57,  0, 35,
                               62, 31, 40,  4, 49,  5, 52, 26,
                               60,  6, 23, 44, 46, 27, 56, 16,
                                7, 39, 48, 24, 59, 14, 12, 55,
                               38, 28, 58, 20, 37, 17, 36, 8];
    board ^= board-1;
    let fold : u64 = (board & 0xffffffff) ^ (board >> 32);
    return TABLE[(((fold * 0x783a9b23) >> 26) & 0x3f) as usize];
}


fn neighborhood(board : u64)->u64 {
    let neighbor : u64 = board | ((board << 1) & !FILE_A) | ((board >> 1) & !FILE_H);
    return (neighbor << 8) | (neighbor >> 8) | neighbor;
}


fn available_moves(o : Othello, turn : bool)->u64 {
    let mut moves : u64 = 0x0;
    let empty = !(o.white|o.black);
    //TODO: this needs to be fixed!!!
    if turn {
        let mut temp = neighborhood(o.black) & empty;
        while temp != 0 {
            let x : usize = debruins(temp);
            if (FLIP_MASK[x] & o.white) != 0 {
                moves |= 1 << x;
            }
            temp &= temp-1;
        }
    } else {
        let mut temp = neighborhood(o.white) & empty;
        while temp != 0 {
            let x : usize = debruins(temp);
            if (FLIP_MASK[x] & o.black) != 0 {
                moves |= 1 << x;
            }
            temp &= temp-1;
        }
    }
    return moves;
}

#[cfg(test)]
mod test_bits {
    use super::*;
    
    #[test]
    fn test_debruins() {
        for i in 0..64 {
            let x : u64 = 1<<i;
            assert_eq!(i, debruins(x));
        }
    }

    #[test]
    fn test_bit_constants() {
        assert_eq!(A1|B1|C1|D1|E1|F1|G1|H1, 0xff);
        assert_eq!(A2|B2|C2|D2|E2|F2|G2|H2, 0xff00);
        assert_eq!(A3|B3|C3|D3|E3|F3|G3|H3, 0xff0000);
        assert_eq!(A4|B4|C4|D4|E4|F4|G4|H4, 0xff000000);
        assert_eq!(A5|B5|C5|D5|E5|F5|G5|H5, 0xff00000000);
        assert_eq!(A6|B6|C6|D6|E6|F6|G6|H6, 0xff0000000000);
        assert_eq!(A7|B7|C7|D7|E7|F7|G7|H7, 0xff000000000000);
        assert_eq!(A8|B8|C8|D8|E8|F8|G8|H8, 0xff00000000000000);
        assert_eq!(RANK_1, 0xff);
        assert_eq!(RANK_2, 0xff00);
        assert_eq!(RANK_3, 0xff0000);
        assert_eq!(RANK_4, 0xff000000);
        assert_eq!(RANK_5, 0xff00000000);
        assert_eq!(RANK_6, 0xff0000000000);
        assert_eq!(RANK_7, 0xff000000000000);
        assert_eq!(RANK_8, 0xff00000000000000);
        assert_eq!(FILE_A, 0x0101010101010101);
        assert_eq!(FILE_B, 0x0202020202020202);
        assert_eq!(FILE_C, 0x0404040404040404);
        assert_eq!(FILE_D, 0x0808080808080808);
        assert_eq!(FILE_E, 0x1010101010101010);
        assert_eq!(FILE_F, 0x2020202020202020);
        assert_eq!(FILE_G, 0x4040404040404040);
        assert_eq!(FILE_H, 0x8080808080808080);
        assert_eq!(RANK_1|RANK_2|RANK_3|RANK_4|RANK_5|RANK_6|RANK_7|RANK_8, 0xffffffffffffffff);
        assert_eq!(FILE_A|FILE_B|FILE_C|FILE_D|FILE_E|FILE_F|FILE_G|FILE_H, 0xffffffffffffffff);
    }

    #[test]
    fn test_available_moves() {
        let y = available_moves(START_POSITION, true);
        let x : u64 = C5|D6|F4|E3;
        println!("Available:\n\n{}\n\nActual:\n\n{}\n\n", y, x);
    }
}







