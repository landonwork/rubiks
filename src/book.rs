//! A construct that represents accumulated knowledge of the Rubik's cube group. This will be the
//! starting point for creating a training dataset for an agent.

#![allow(private_bounds)]
use std::{borrow::Borrow, cmp::PartialOrd, io, marker::PhantomData};

use sled::{self, Db, Tree};

use crate::{
    action::{Action, Move, QuarterTurn, Turn},
    cubelet::Rotation,
    word::Word
};

fn as_bytes<T>(slice: &[T]) -> &[u8] {
    let ptr: *const _ = slice;
    unsafe { std::slice::from_raw_parts(ptr.cast(), std::mem::size_of::<T>()) }
}

pub trait Int: PartialOrd + Copy {
    type ToBytes: Borrow<[u8]>;
    fn to_bytes(&self) -> Self::ToBytes;
    fn from_bytes(bytes: &[u8]) -> Self;
}

impl Int for u8 {
    type ToBytes = [u8; 1];

    fn to_bytes(&self) -> Self::ToBytes {
        self.to_le_bytes()
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        debug_assert_eq!(bytes.len(), 1);
        bytes[0]
    }
}

impl Int for u16 {
    type ToBytes = [u8; 2];

    fn to_bytes(&self) -> Self::ToBytes {
        self.to_le_bytes()
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        u16::from_le_bytes(bytes.try_into().unwrap())
    }
}

impl Int for u32 {
    type ToBytes = [u8; 4];

    fn to_bytes(&self) -> Self::ToBytes {
        self.to_le_bytes()
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        u32::from_le_bytes(bytes.try_into().unwrap())
    }
}

// I think we always store the cube as the key, followed by the depth (u8, u16, u32), followed by
// the word (which has a variable length). There will be a special entry that records the format of
// the Book. On opening an existing Book, it checks the data format and returns an error if the
// format does not match the generics in the tree.
#[derive(Clone)]
pub struct Book<Depth = u16, Action = Turn> {
    // Db struct included to have access to the size_on_disk method
    db: Db,
    inner: Tree,
    // Depth: "this_books_depth_type"
    // Action: "this_books_action_type"
    _phantom: PhantomData<(Depth, Action)>,
}

const DEPTH_ENTRY: &[u8] = b"this_books_depth_type";
const ACTION_ENTRY: &[u8] = b"this_books_action_type";

impl<D: Int, A: Packable + Into<Move>> Book<D, A> {
    pub fn open(file_path: &str) -> io::Result<Self> {
        let db = sled::open(file_path)?;
        if !db.was_recovered() {
            let _ = std::fs::remove_dir_all(file_path);
            return Err(io::Error::new(io::ErrorKind::NotFound, file_path.to_owned()));
        }

        let inner = db.open_tree(b"book")?;

        let depth_type = std::str::from_utf8(
            inner.get(DEPTH_ENTRY)?
                .ok_or(io::Error::new(io::ErrorKind::NotFound, "Opened book does not contain a depth type"))?
                .as_ref()
            )
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8 found in depth type entry"))?
            .to_owned();
        if std::str::from_utf8(depth_type.as_ref()).unwrap() != std::any::type_name::<D>() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Opened book has a different depth type: expected {}, got {}",
                    std::any::type_name::<D>(),
                    depth_type
                )
            ))
        }

        let action_type = std::str::from_utf8(
            inner.get(ACTION_ENTRY)?
                .ok_or(io::Error::new(io::ErrorKind::NotFound, "Opened book does not contain an action type"))?
                .as_ref()
            )
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8 found in action type entry"))?
            .to_owned();
        if std::str::from_utf8(action_type.as_ref()).unwrap() != std::any::type_name::<A>() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Opened book has a different action type: expected {}, got {}",
                    std::any::type_name::<A>(),
                    action_type
                )
            ))
        }

        Ok(Book { db, inner, _phantom: PhantomData })
    }

    pub fn create(file_path: &str) -> io::Result<Self> {
        let db = sled::open(file_path)?;
        if db.was_recovered() { return Err(io::Error::new(io::ErrorKind::AlreadyExists, file_path.to_owned())); }

        let inner = db.open_tree(b"book")?;
        inner.insert(DEPTH_ENTRY, std::any::type_name::<D>())?;
        inner.insert(ACTION_ENTRY, std::any::type_name::<A>())?;

        Ok(Book { db, inner, _phantom: PhantomData })
    }

    pub fn insert(&self, word: Word<A>, depth: D) -> io::Result<Option<D>> {
        // TODO: pack if packed; is packed part of the generics or is it a runtime setting?
        // Probably the generics, right?
        // let key = as_bytes(&pair.current_state().cubelets);
        let key = pack(&word.cube.cubelets);

        let update_fn = |slice: Option<&[u8]>| -> Option<Vec<u8>> {
        let depth = if let Some(slice) = slice {
                let current = D::from_bytes(slice);
                if current < depth {
                    depth
                } else {
                    current
                }
            } else {
                depth
            };

            Some(depth.to_bytes().borrow().to_vec())
        };

        let previous = self.inner.fetch_and_update(key, update_fn)?;

        Ok(previous.map(|ivec| D::from_bytes(ivec.as_ref())))
    }

    pub fn size(&self) -> io::Result<u64> {
        Ok(self.db.size_on_disk()?)
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
    // 24 rotations < 32
    const PACKED_BITS: usize = 5;
}

impl Packable for Move {
    // 45 moves < 64
    const PACKED_BITS: usize = 6;

    fn to_byte(&self) -> u8 {
        Move::ALL.iter().position(|x| x == self).unwrap() as u8
    }

    fn from_byte(byte: u8) -> Option<Self> {
        Move::ALL.get(byte as usize).copied()
    }
}

impl Packable for Turn {
    // 18 turns < 32
    const PACKED_BITS: usize = 5;
}

impl Packable for QuarterTurn {
    // 12 turns < 16
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
    use super::*;
    use crate::cubelet::Axis;

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

    #[test]
    fn test_create_and_open_book() {
        const NAME: &str = "test_create_book";
        let _ = std::fs::remove_dir_all(NAME);

        let res1: Result<Book<u16, Move>, _> = Book::open(NAME);
        assert!(res1.is_err());
        assert!(!std::path::Path::new(NAME).exists());

        let new_book: Book<u16, Move> = Book::create(NAME).unwrap();
        let res2: Result<Book<u16, Move>, _> = Book::create(NAME);
        assert!(res2.is_err());

        drop(new_book);
        let _ = std::fs::remove_dir_all(NAME);
        assert!(!std::path::Path::new(NAME).exists());
    }
}
