#![allow(dead_code)]

const fn generate_crc_table() -> [u64; 256] {
    let mut res = [0_u64; 256];
    let (mut n, mut k) = (0, 0);
    loop {
        let mut c: u64 = n;
        loop {
            c = match c & 1 {
                1 => 0xedb88320_u64 ^ (c >> 1),
                _ => c >> 1,
            };
            k += 1;
            if k >= 8 {
                break;
            }
        }
        res[n as usize] = c;
        n += 1;
        if n >= 256 {
            break;
        }
    }
    res
}

const fn generate_fixed_huffman_code_int_sequence() -> [u16; 288] {
    let mut res = [0_u16; 288];
    let mut i = 0;
    loop {
        if i >= 288 {
            break;
        }
        res[i] = match i {
            256..=279 => 7,
            0..=143 | 280.. => 8,
            144..=255 => 9,
        };
        i += 1;
    };
    res
}

const fn get_bl_count<const N: usize, const MAX_BITS: usize>(int_sequence: [u16; N]) -> [u16; MAX_BITS] {
    let mut res = [0; MAX_BITS];
    let mut i = 0;
    loop {
        if i >= N {
            break;
        }
        res[int_sequence[i] as usize - 1] += 1;
        i += 1;
    }
    res
}

const fn generate_huffman_tree<const MAX_BITS: usize, const SIZE: usize>(int_sequence: [u16; SIZE]) -> [u16; SIZE] {
    // 1. Generate amount of symbols using each number of bits
    let mut bl_count = [0; MAX_BITS];
    let mut i = 0;
    loop {
        if i >= SIZE {
            break;
        }
        bl_count[int_sequence[i] as usize - 1] += 1;
        i += 1;
    }

    // 2. Generate next code for all bit sizes
    let mut next_code = [0; MAX_BITS];
    let (mut bits, mut code) = (1, 0);
    loop {
        if bits >= MAX_BITS {
            break;
        }
        code = (code + bl_count[bits - 1]) << 1;
        next_code[bits] = code;
        bits += 1;
    }

    // 3. Assign values to all symbols.
    let mut n = 0;
    let mut res = [0; SIZE];
    loop {
        if n >= SIZE {
            break;
        }
        let len = int_sequence[n] - 1;
        res[n] = next_code[len as usize];
        next_code[len as usize] += 1;
        n += 1;
    }
    res
}

const CRC_TABLE: [u64; 256] = generate_crc_table();
const FIXED_HUFFMAN_TREE: [u16; 288] = generate_huffman_tree::<9, 288>(generate_fixed_huffman_code_int_sequence());

enum FilterType {
    None = 0,
    Sub,
    Up,
    Average,
    Paeth
}

enum InterlacingMode {
    Mode0 = 0,
    Mode1
}

const ADAM_7_PATTERN: [[u8; 8]; 8] = [
    [1, 6, 4, 6, 2, 6, 4, 6],
    [7; 8],
    [5, 6, 5, 6, 5, 6, 5, 6],
    [7; 8],
    [3, 6, 4, 6, 3, 6, 4, 6],
    [7; 8],
    [5, 6, 5, 6, 5, 6, 5, 6],
    [7; 8],
];

fn paeth_predictor(a: u8, b: u8, c: u8) -> u8 {
    let (a, b, c) = (a as i16, b as i16, c as i16);
    let p: i16 = a + b - c;
    let (pa, pb, pc): (i16, i16, i16) = ((p - a).abs(), (p - b).abs(), (p - c).abs());
    match pa <= pb && pa <= pc {
        true => pa as u8,
        false => match pb <= pc {
            true => pb as u8,
            false => pc as u8,
        },
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
