//! A construct that represents accumulated knowledge of the Rubik's cube group. This will be the
//! starting point for creating a training dataset for an agent.

use std::{io, marker::PhantomData};

use sled::{self, Db, IVec, Tree};

use crate::prelude::*;

// fn as_bytes<T>(slice: &[T]) -> &[u8] {
//     let ptr: *const _ = slice;
//     unsafe { std::slice::from_raw_parts(ptr.cast(), std::mem::size_of::<T>() * slice.len()) }
// }

// I think we always store the cube as the key, followed by the depth (u8, u16, u32).
// There will be a special entry that records the format of
// the Book. On opening an existing Book, it checks the data format and returns an error if the
// format does not match the generics in the tree.
#[derive(Clone)]
pub struct Book<Action = Turn> {
    // Db struct included to have access to the size_on_disk method
    db: Db,
    inner: Tree,
    // Depth: "this_books_depth_type"
    // Action: "this_books_action_type"
    _phantom: PhantomData<Action>,
}

const ACTION_ENTRY: &[u8] = b"this_books_action_type";
const PACKED_ENTRY: &[u8] = b"this_books_packed_attribute";

#[allow(private_bounds)]
impl<A: Action + Packable> Book<A> {
    pub fn open(file_path: &str) -> io::Result<Self> {
        let db = sled::open(file_path)?;
        if !db.was_recovered() {
            let _ = std::fs::remove_dir_all(file_path);
            return Err(io::Error::new(io::ErrorKind::NotFound, file_path.to_owned()));
        }

        let inner = db.open_tree(b"book")?;

        // Check action type
        let action_type = std::str::from_utf8(
            inner.get(ACTION_ENTRY)?
                .ok_or(io::Error::new(io::ErrorKind::NotFound, "Opened book does not contain an action type"))?
                .as_ref()
            )
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8 found in action type entry"))?
            .to_owned();

        if &action_type != std::any::type_name::<A>() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Opened book has a different action type: expected {}, got {}",
                    std::any::type_name::<A>(),
                    action_type
                )
            ))
        }

        // Check packed attribute
        let packed_attr = std::str::from_utf8(
            inner.get(PACKED_ENTRY)?
                .ok_or(io::Error::new(io::ErrorKind::NotFound, "Opened book does not contain a packed attribute"))?
                .as_ref()
            )
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8 found in packed attribute"))?
            .to_owned();

        if &packed_attr != "false" {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Opened book has a different packed attribute: expected {}, got {}",
                    false,
                    packed_attr
                )
            ))
        }

        let book = Book { db, inner, _phantom: PhantomData };
        book.get_depth(&Cube::<Position>::default())?
            .ok_or(io::Error::new(
                io::ErrorKind::NotFound,
                "Book does not contain the solved cube".to_owned()
            ))?;

        Ok(book)
    }

    pub fn create(file_path: &str) -> io::Result<Self> {
        let db = sled::open(file_path)?;
        if db.was_recovered() { return Err(io::Error::new(io::ErrorKind::AlreadyExists, file_path.to_owned())); }

        let inner = db.open_tree(b"book")?;
        inner.insert(ACTION_ENTRY, std::any::type_name::<A>())?;
        let this = Self { db, inner, _phantom: PhantomData }; 
        this.insert_cube(&Cube::default(), 0u8.into())?;

        Ok(this)
    }

    pub fn create_or_open(file_path: &str) -> io::Result<Self> {
        if std::path::Path::new(file_path).exists() {
            Self::open(file_path)
        } else {
            Self::create(file_path)
        }
    }

    pub fn contains(&self, cube: &Cube<Position>) -> io::Result<bool> {
        Ok(self.inner.get(pack(&cube.cubelets))?.is_some())
    }

    pub fn get_depth(&self, cube: &Cube<Position>) -> io::Result<Option<u8>> {
        let depth = self.inner.get(pack(&cube.cubelets))?;
        Ok(depth.map(|ivec| ivec[0]))
    }

    /// Insert a cube and a depth into the book. If the book has no record of the cube, the cube
    /// and depth are inserted. If the book has an existing record, it is only replaced if the
    /// passed depth is less than the existing depth. The existing depth is returned if there is
    /// one.
    pub fn insert_cube(&self, cube: &Cube<Position>, depth: u8) -> io::Result<Option<u8>> {
        let key = pack(&cube.cubelets);

        let update_fn = |slice: Option<&[u8]>| -> Option<_> {
            let depth = slice
                .map(|bytes| { std::cmp::min(bytes[0], depth) })
                .unwrap_or(depth);
            println!("{:?}, {:?}", slice, &[depth]);
            Some(IVec::from(&[depth]))
        };

        let previous = self.inner.fetch_and_update(key, update_fn)?;

        Ok(previous.map(|ivec| ivec[0]))
    }

    /// Insert a cube into the book and if the previous recorded depth was overwritten,
    /// update all the neighbors and recurse.
    pub fn update_cube(&self, cube: &Cube<Position>, depth: u8) -> io::Result<()> {
        match self.insert_cube(&cube, depth)? {
            Some(old_depth) if depth < old_depth => {
                match A::ALL.into_iter()
                    .find_map(|m| {
                        let new_cube = cube.clone().make_move(*m);
                        self.update_cube(&new_cube, depth + 1).err()
                    })
                {
                    Some(error) => Err(error),
                    None => Ok(())
                }
            }
            None | Some(_) => Ok(()),
        }
    }

    /// Run update_cube for all cubes along the path of the given word.
    pub fn update_word(&self, word: &Word<A>) -> io::Result<()> {
        let mut depth = 0;
        let mut cube = Cube::default();

        for action in word.as_actions() {
            cube = cube.make_move(action);
            depth += 1;
            match self.insert_cube(&cube, depth)? {
                // Copied straight from update_cube
                Some(old_depth) if depth < old_depth => {
                    match A::ALL.into_iter()
                        .find_map(|m| {
                            let new_cube = cube.clone().make_move(*m);
                            self.update_cube(&new_cube, depth + 1).err()
                        })
                    {
                        Some(error) => { return Err(error); }
                        None => {}
                    }
                }
                // If we come across a cube that has a lower depth than we expect,
                // we correct our depth and update everything behind.
                Some(old_depth) if depth > old_depth => {
                    depth = old_depth;
                    self.update_cube(&cube.clone().make_move(action.inverse()), depth + 1)?;
                }
                None | Some(_) => {},
            }
        }

        Ok(())
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
#[allow(dead_code)]
fn unpack<T: Packable>(bytes: &[u8]) -> Vec<T> {
    debug_assert!(0 < T::PACKED_BITS && T::PACKED_BITS < 8);

    let size = bytes.len() * 8 / T::PACKED_BITS;
    // println!("size: {size}");
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
        #[allow(dead_code)]
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

        let res1: Result<Book<Move>, _> = Book::open(NAME);
        assert!(res1.is_err());
        assert!(!std::path::Path::new(NAME).exists());

        let new_book: Book<Move> = Book::create(NAME).unwrap();
        let res2: Result<Book<Move>, _> = Book::create(NAME);
        assert!(res2.is_err());

        drop(new_book);
        let _ = std::fs::remove_dir_all(NAME);
        assert!(!std::path::Path::new(NAME).exists());
    }

    #[test]
    fn test_insert() {
        const NAME: &str = "test_insert";
        let _ = std::fs::remove_dir_all(NAME);
        let book: Book<Move> = Book::create(NAME).unwrap();
        let mut cube = Cube::default();
        cube = cube.make_move(Move(Axis::X, 0, 1));
        book.insert_cube(&cube, 1).unwrap();
        cube = cube.make_move(Move(Axis::X, 0, 2));
        book.insert_cube(&cube, 2).unwrap();
        let cube3 = cube.clone().make_move(Move(Axis::Y, 0, 1));
        book.insert_cube(&cube3, 3).unwrap();

        assert_eq!(Some(3), book.get_depth(&cube3).unwrap());
        assert_eq!(Some(2), book.get_depth(&cube).unwrap());

        book.update_cube(&cube, 1).unwrap();

        assert_eq!(Some(2), book.get_depth(&cube3).unwrap());
        assert_eq!(Some(1), book.get_depth(&cube).unwrap());
    }

    #[test]
    fn test_insert_word() {
        const NAME: &str = "test_insert_word";
        let _ = std::fs::remove_dir_all(NAME);
        let book: Book<Turn> = Book::create(NAME).unwrap();
        let word = Word::from([Turn::R, Turn::R, Turn::R, Turn::U]);

        book.update_word(&word).unwrap();
        assert_eq!(Some(4), book.get_depth(word.cube()).unwrap());

        let normal = word.normal_form();
        println!("{:?}", normal.actions);
        book.update_word(&normal).unwrap();
        assert_eq!(Some(1), book.get_depth(&Cube::default().make_move(Turn::R3)).unwrap());
        assert_eq!(Some(2), book.get_depth(normal.cube()).unwrap());
    }

    #[test]
    fn test_update_word_backtrack() {
        const NAME: &str = "test_update_word_backtrack";
        let _ = std::fs::remove_dir_all(NAME);
        let book: Book<Turn> = Book::create(NAME).unwrap();

        let mut word = Word::new();
        let mut mid18 = Word::new();
        let mut count = 0;
        for _ in 0..6 {
            for t in [Turn::R3, Turn::D3, Turn::R, Turn::D] {
                word.make_move(t);
                count += 1;
                if count == 18 {
                    mid18 = word.clone();
                }
            }
        }

        book.update_word(&word).unwrap();
        assert_eq!(Cube::default(), word.cube);
        assert_eq!(Some(6), book.get_depth(&mid18.cube).unwrap());
    }
}
