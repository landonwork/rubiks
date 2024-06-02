//! A construct that represents accumulated knowledge of the Rubik's cube group. This will be the
//! starting point for creating a training dataset for an agent.

use sled::Tree;

use crate::{
    action::{Move, Turn, QuarterTurn},
    cubelet::{Axis, Rotation}
};

pub trait Book {}

#[derive(Clone)]
pub struct DepthBook(pub(crate) Tree);

impl DepthBook {
    fn close(self) -> Result<(), ()> {
        todo!()
    }
}

trait Packable: Copy {
    const PACKED_BITS: usize;
    const PAD: u8 = u8::MAX >> (8 - Self::PACKED_BITS as u8);

    #[inline]
    fn to_byte(&self) -> u8 {
        // It turns out that transmute_copy does not care if the two types have the same size which
        // is both useful in this instance and scary.
        debug_assert_eq!(std::mem::size_of::<Self>(), std::mem::size_of::<u8>());
        unsafe { std::mem::transmute_copy(self) }
    }

    #[inline]
    fn from_byte(byte: u8) -> Option<Self> {
        // It turns out that transmute_copy does not care if the two types have the same size which
        // is both useful in this instance and scary.
        debug_assert_eq!(std::mem::size_of::<Self>(), std::mem::size_of::<u8>());
        if byte == Self::PAD {
            None
        } else {
            unsafe { Some(std::mem::transmute_copy(&byte)) }
        }
    }
}

impl Packable for Rotation {
    // 24 rotations
    const PACKED_BITS: usize = 5;
}

impl Packable for Move {
    // 45 moves
    const PACKED_BITS: usize = 6;

    fn to_byte(&self) -> u8 {
        Move::ALL.iter().position(|x| x == self).unwrap() as u8
    }

    fn from_byte(byte: u8) -> Option<Self> {
        Move::ALL.get(byte as usize).copied()
    }
}

impl Packable for Turn {
    // 18 turns
    const PACKED_BITS: usize = 5;
}

impl Packable for QuarterTurn {
    // 12 turns
    const PACKED_BITS: usize = 4;
}

/// Pack a byte slice into a smaller byte slice.
/// `bytes` is the slice to be packed and `packed_bits` is the number of bits that each byte should be
/// packed into (between 1 and 7).
fn pack<T: Packable>(values: &[T]) -> Vec<u8> {
    debug_assert!(0 < T::PACKED_BITS && T::PACKED_BITS < 8);

    let new_bits = values.len() * T::PACKED_BITS;
    let size = new_bits / 8 + (new_bits % 8 > 0) as usize;
    let mut new = vec![0; size];

    let mut bit = 0;
    let mut value_i = 0;
    while bit <= size * 8 - T::PACKED_BITS {
        let end_bit = bit + T::PACKED_BITS;
        let i = bit / 8;
        let bit_i = bit % 8;
        let value = values.get(value_i).map(|x| x.to_byte()).unwrap_or(T::PAD);

        if i == (end_bit - 1) / 8 {
            new[i] |= value << (8 - bit_i - T::PACKED_BITS);
        } else {
            let end_bit_i = end_bit % 8;
            new[i]   |= value >> (bit_i + T::PACKED_BITS - 8);
            new[i+1] |= value << (8 - end_bit_i);
        }

        bit += T::PACKED_BITS;
        value_i += 1;
    }

    new
}

/// Unpack a byte slice into a smaller byte slice.
/// `bytes` is the slice to be packed and `packed_bits` is the number of bits that each byte was
/// packed into (between 1 and 7). Warning: if there are enough padding bits at the end to unpack
/// another value, this function WILL unpack it;
fn unpack<T: Packable>(bytes: &[u8]) -> Vec<T> {
    debug_assert!(0 < T::PACKED_BITS && T::PACKED_BITS < 8);

    let size = bytes.len() * 8 / T::PACKED_BITS;
    println!("size: {size}");
    let mut new = vec![0; size];

    let mut bit = 0;
    for byte in new.iter_mut() {
        let end_bit = bit + T::PACKED_BITS;
        let i = bit / 8;
        let bit_i = bit % 8;

        if i == (end_bit - 1) / 8 {
            *byte = (u8::MAX >> (8 - T::PACKED_BITS)) & (bytes[i] >> (8 - bit_i - T::PACKED_BITS));
        } else {
            let end_bit_i = end_bit % 8;
            *byte = (((u8::MAX >> bit_i) & bytes[i]) << end_bit_i) | (bytes[i+1] >> (8 - end_bit_i));
        }

        bit += T::PACKED_BITS;
    }

    new.into_iter().filter_map(T::from_byte).collect()
}


#[cfg(test)]
mod tests {
    #![allow(dead_code)]
    use super::*;

    // All of these tests should be of a length that requires padding at the end of the packed
    // version
    #[test]
    fn test_pack() {
        #[derive(Copy, Clone)]
        struct A(u8);
        impl Packable for A { const PACKED_BITS: usize = 3; }

        let bytes = [A(0b000), A(0b001), A(0b010), A(0b011), A(0b100), A(0b101)];
        let expected = [0b00000101, 0b00111001, 0b01111111];
        let actual = pack(bytes.as_slice());
        assert_eq!(expected.as_slice(), actual.as_slice());
    }

    #[test]
    fn test_unpack() {
        #[derive(Copy, Clone, Debug, PartialEq, Eq)]
        struct A(u8);
        impl Packable for A { const PACKED_BITS: usize = 3; }

        let expected = [A(0b000), A(0b001), A(0b010), A(0b011), A(0b100), A(0b101)];
        let bytes = [0b00000101, 0b00111001, 0b01111111];
        let actual = unpack(bytes.as_slice());
        assert_eq!(expected.as_slice(), actual.as_slice());
    }

    #[test]
    fn test_pack_unpack_move() {
        let input = vec![Move(Axis::X, 1, 1), Move(Axis::Y, 1, 3), Move(Axis::X, 0, 1)];
        let output = unpack(&pack(&input));
        assert_eq!(input, output);
    }

    #[test]
    fn test_pack_unpack_turn() {
        let input = vec![Turn::L, Turn::B, Turn::R3, Turn::D3, Turn::F2];
        let output = unpack(&pack(&input));
        assert_eq!(input, output);
    }

    #[test]
    fn test_pack_unpack_quarter_turn() {
        let input = vec![QuarterTurn::L, QuarterTurn::B, QuarterTurn::R3, QuarterTurn::D3, QuarterTurn::F];
        let output = unpack(&pack(&input));
        assert_eq!(input, output);
    }

    #[test]
    fn test_pack_unpack_rotation() {
        let input = vec![Rotation::X, Rotation::XY3, Rotation::Z2, Rotation::Y3, Rotation::X3Z3];
        let output = unpack(&pack(&input));
        assert_eq!(input, output);
    }
}
