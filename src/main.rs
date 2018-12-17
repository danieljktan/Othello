use std::fmt;

fn main() {

}

enum Turn {
    WHITE,
    BLACK,
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

const FILE : [u64; 8]  = [FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H];
const RANK : [u64; 8]  = [RANK_1, RANK_2, RANK_3, RANK_4, RANK_5, RANK_6, RANK_7, RANK_8];
const DIAG : [u64; 16] = [A1_H8, A2_G8, A3_F8, A4_E8, A5_D8, A6_C8, A7_B8,    H1,
                            0x0,    A8, G1_H2, F1_H3, E1_H4, D1_H5, C1_H6, B1_H7];
const ADIA : [u64; 16] = [A8_H1, A7_G1, A6_F1, A5_E1, A4_D1, A3_C1, A2_B1,    A1, 
                            0x0,    H8, G8_H7, F8_H6, E8_H5, D8_H4, C8_H3, B8_H2];

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
                                 38, 28, 58, 20, 37, 17, 36,  8];
    board ^= board-1;
    let fold : u64 = (board & 0xffffffff) ^ (board >> 32);
    return TABLE[((fold * 0x783a9b23 >> 26) & 0x3f) as usize];
}


fn neighborhood(board : u64)->u64 {
    let neighbor : u64 = board | ((board << 1) & !FILE_A) | ((board >> 1) & !FILE_H);
    return (neighbor << 8) | (neighbor >> 8) | neighbor;
}


fn reverse_bits(mut n : u64)->u64 {
    let mut ret : u64 = 0;
    while n != 0 {
        ret |= 1 << (63 - debruins(n));
        n &= n-1;
    }
    return ret;
}


//hyperbola quintenssence
fn hq(o : u64, m : u64, s : u64)->u64 {
    let x : u64 = o & m;
    return ((x.wrapping_sub(s << 1)) ^ reverse_bits(reverse_bits(x).wrapping_sub(reverse_bits(s) << 1))) & m;
}


fn sliding_mask(x : usize, o : u64, bit : u64)->u64 {
    assert_eq!(1 << x, bit);
    let rank : usize = x >> 3;
    let file : usize = x & 0x7;
    let horizontal    : u64 = hq(o, FILE[file], bit);
    let vertical      : u64 = hq(o, RANK[rank], bit);
    let diagonal      : u64 = hq(o, DIAG[rank.wrapping_sub(file) & 0xf], bit);
    let anti_diagonal : u64 = hq(o, ADIA[(rank+file) ^ 0x7], bit);
    return horizontal ^ vertical ^ diagonal ^ anti_diagonal;
}


fn available_moves(o : &Othello, turn : Turn)->u64 {
    //TODO: this whole bit manipulation hack may not be correct...
    //TODO: optimize?
    assert_eq!(o.white & o.black, 0x0);

    let mut moves : u64 = 0x0;
    let empty = !(o.white|o.black);
    match turn {

    Turn::WHITE=> {
        let mut temp = neighborhood(o.black) & empty;
        while temp != 0 {
            let x : usize = debruins(temp);
            let bit : u64 = 1 << x;
            let y : u64 = sliding_mask(x, o.white|empty, bit);
            if (y & o.black) != 0 && (y & o.white & !neighborhood(bit)) != 0 {
                moves |= bit;
            }
            temp &= temp-1;
        }
    },
    Turn::BLACK=> {
        let mut temp = neighborhood(o.white) & empty;
        while temp != 0 {
            let x : usize = debruins(temp);
            let bit : u64 = 1 << x;
            let y : u64 = sliding_mask(x, o.black|empty, bit);
            if (y & o.white) != 0 && (y & o.black & !neighborhood(bit)) != 0 {
                moves |= bit;
            }
            temp &= temp-1;
        }
    }
    }

    return moves;
}


fn evaluate(o : &Othello, at_end : bool)->i64 {
    let w : i64 = o.white.count_ones() as i64;
    let b : i64 = o.black.count_ones() as i64;
    if at_end {
        if w == b {
            return 0;
        } else if w > b {
            return i64::min_value();
        } else {
            return i64::max_value();
        }
    } else {
        return b - w;
    }
}


fn make_move(o : &Othello, tile : u64, turn : Turn)-> Othello {
    assert_eq!(tile & (tile-1), 0x0); //only 1 bit of data
    assert_eq!((o.white|o.black) & tile, 0x0); //othello board must be valid
    assert_eq!(o.white & o.black, 0x0);

    match turn {

    Turn::BLACK=> {
        let mut black : u64 = o.black | tile;
        let mut white : u64 = o.white;

        return Othello{white:white, black:black};
    },

    Turn::WHITE=> {
        let mut white : u64 = o.white | tile;
        let mut black : u64 = o.black;

        return Othello{white:white, black:black};
    }

    }
}

fn minimax(o : Othello, depth : i64, mut alpha : i64, mut beta : i64, turn : Turn)->(u64, i64) {
    assert_eq!(o.white & o.black, 0x0);
    assert!(depth >= 0);

    let mut w = available_moves(&o, Turn::WHITE);
    let mut b = available_moves(&o, Turn::BLACK);
    if depth == 0 || (w == 0 && b == 0){
       return (0x0, evaluate(&o, w == 0 && b == 0));
    }

    match turn {
            
    Turn::BLACK=> {
        let mut best_mov : u64 = 0x0;
        let mut best_value : i64 = i64::min_value();
        while b != 0 {
            let tile : u64 = 1 << debruins(b);
            let (mov, val) = minimax(make_move(&o, tile, Turn::BLACK), depth-1, alpha, beta, Turn::WHITE);
            if val > best_value {
                best_value = val;
                best_mov = tile;
            }

            if best_value >= alpha {
                alpha = best_value;
            }

            if alpha >= beta {
                break;
            }

            b &= b-1;
        }
        return (best_mov, best_value);
    },

    Turn::WHITE=> {
        let mut best_mov : u64 = 0x0;
        let mut best_value : i64 = i64::max_value();
        while w != 0 {
            let tile : u64 = 1 << debruins(w);
            let (mov, val) = minimax(make_move(&o, tile, Turn::WHITE), depth-1, alpha, beta, Turn::BLACK);
            if val < best_value {
                best_value = val;
                best_mov = tile;
            }

            if best_value <= alpha {
                alpha = best_value;
            }

            if alpha <= beta {
                break;
            }

            w &= w-1;
        }
        return (best_mov, best_value);
    }

    }
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
        const POS1 : Othello = Othello{white:B2, black:C1};
        const POS2 : Othello = Othello{white:D8, black:E7|F6|G5};
        const POS3 : Othello = Othello{white:B2|C3|D4|E5|F6|G7, black:A1};
        const POS4 : Othello = Othello{white:D4, black:C4|D5|D6|D3|D2|E4|F4};
        
        assert_eq!(available_moves(&START_POSITION, Turn::WHITE), C5|D6|F4|E3);
        assert_eq!(available_moves(&START_POSITION, Turn::BLACK), C4|D3|F5|E6);
        assert_eq!(available_moves(&POS1, Turn::BLACK), A3);
        assert_eq!(available_moves(&POS2, Turn::WHITE), H4);
        assert_eq!(available_moves(&POS3, Turn::BLACK), H8);
        assert_eq!(available_moves(&POS4, Turn::WHITE), B4|G4|D1|D7);
        assert_eq!(available_moves(&POS4, Turn::BLACK), 0x0);
    }
}







