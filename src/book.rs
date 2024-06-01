//! A construct that represents accumulated knowledge of the Rubik's cube group. This will be the
//! starting point for creating a training dataset for an agent.

use std::sync::Arc;
use sled::Tree;

#[derive(Clone)]
pub struct Book(pub Arc<Tree>);



/// Pack a byte slice into a smaller byte slice.
/// `bytes` is the slice to be packed and `packed_bits` is the number of bits that each byte should be
/// packed into (between 1 and 7).
fn pack(bytes: &[u8], packed_bits: usize) -> Vec<u8> {
    debug_assert!(0 < packed_bits && packed_bits < 8);

    let new_bits = bytes.len() * packed_bits;
    let size = new_bits / 8 + (new_bits % 8 > 0) as usize;
    let mut new = vec![0; size];

    let mut bit = 0;
    for byte in bytes {
        let end_bit = bit + packed_bits;
        let i = bit / 8;
        let bit_i = bit % 8;
        if i == (end_bit - 1) / 8 {
            new[i] |= byte << (8 - bit_i - packed_bits);
        } else {
            let end_bit_i = end_bit % 8;
            new[i]   |= byte >> (bit_i + packed_bits - 8);
            new[i+1] |= byte << (8 - end_bit_i);
        }

        bit += packed_bits;
    }

    new
}

/// Unpack a byte slice into a smaller byte slice.
/// `bytes` is the slice to be packed and `packed_bits` is the number of bits that each byte was
/// packed into (between 1 and 7). Warning: if there are enough padding bits at the end to unpack
/// another value, this function WILL unpack it;
fn unpack(bytes: &[u8], packed_bits: usize) -> Vec<u8> {
    debug_assert!(0 < packed_bits && packed_bits < 8);

    let size = bytes.len() * 8 / packed_bits;
    println!("size: {size}");
    let mut new = vec![0; size];

    let mut bit = 0;
    for byte in new.iter_mut() {
        let end_bit = bit + packed_bits;
        let i = bit / 8;
        let bit_i = bit % 8;

        if i == (end_bit - 1) / 8 {
            *byte = (u8::MAX >> (8 - packed_bits)) & (bytes[i] >> (8 - bit_i - packed_bits));
        } else {
            let end_bit_i = end_bit % 8;
            *byte = (((u8::MAX >> bit_i) & bytes[i]) << end_bit_i) | (bytes[i+1] >> (8 - end_bit_i));
        }

        bit += packed_bits;
    }

    new
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pack() {
        let bytes = [0b00, 0b01, 0b10, 0b11];
        let expected = [0b00011011];
        let actual = pack(bytes.as_slice(), 2);
        assert_eq!(expected.as_slice(), actual.as_slice());
    }

    #[test]
    fn test_pack_long() {
        let bytes = [0b000, 0b001, 0b010, 0b011, 0b100, 0b101, 0b110, 0b111];
        let expected = [0b00000101, 0b00111001, 0b01110111];
        let actual = pack(bytes.as_slice(), 3);
        assert_eq!(expected.as_slice(), actual.as_slice());
    }

    #[test]
    fn test_unpack() {
        let bytes = [0b00011011];
        let expected = [0b00, 0b01, 0b10, 0b11];
        let actual = unpack(bytes.as_slice(), 2);
        assert_eq!(expected.as_slice(), actual.as_slice());
    }

    #[test]
    fn test_unpack_long() {
        let bytes = [0b00000101, 0b00111001, 0b01110111];
        let expected = [0b000, 0b001, 0b010, 0b011, 0b100, 0b101, 0b110, 0b111];
        let actual = unpack(bytes.as_slice(), 3);
        assert_eq!(expected.as_slice(), actual.as_slice());
    }

}
