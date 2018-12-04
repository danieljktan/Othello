use std::fmt;

fn main() {
    println!("{}", START_POSITION);
}


const START_POSITION : Othello = Othello{ white: (1<<28)|(1<<35), black:(1<<27)|(1<<36)};


struct Othello {
    white : u64,
    black : u64,
}


impl fmt::Display for Othello {
    fn fmt(&self, f: &mut fmt::Formatter)->fmt::Result {
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


fn debruins(mut board : u64)->u64 {
    const TABLE : [u64; 64] = [63, 30,  3, 32, 25, 41, 22, 33,
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

#[cfg(test)]
mod tests {
    use super::*;
    
    //unit test for debruins multiplication.
    #[test]
    fn test_debruins() {
        for i in 0..64 {
            let x : u64 = 1<<i;
            assert_eq!(i, debruins(x));
        }
    }
}








