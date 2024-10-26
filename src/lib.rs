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
const CRC_TABLE: [u64; 256] = generate_crc_table();

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
