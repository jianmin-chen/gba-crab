// Set of utils, primarily for working with bytes.

pub fn masked(byte: u8, bit: u8) -> u8 {
    (byte >> bit) & 1
}

pub fn signed(unsigned: u8) -> i8 {
    if unsigned < 128 {
        return unsigned as i8;
    }

    ((unsigned as isize) - 256) as i8
}
