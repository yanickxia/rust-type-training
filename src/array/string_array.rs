use crate::array::{Array, ArrayBuilder};
use bitvec::prelude::BitVec;
use crate::array::iterator::ArrayIterator;


/// An [`Array`] that stores [`String`]
pub struct StringArray {
    /// The flattened data of string.
    data: Vec<u8>,

    /// Offsets of each string in the data flat array.
    offsets: Vec<usize>,

    /// The null bitmap of this array.
    bitmap: BitVec,
}

impl Array for StringArray {
    type Builder = StringArrayBuilder;
    type OwnedItem = String;
    type RefItem<'a> = &'a str;

    fn get(&self, idx: usize) -> Option<&str> {
        if self.bitmap[idx] {
            let range = self.offsets[idx]..self.offsets[idx + 1];
            Some(unsafe { std::str::from_utf8_unchecked(&self.data[range]) })
        } else {
            None
        }
    }

    fn len(&self) -> usize {
        self.bitmap.len()
    }

    fn iter(&self) -> ArrayIterator<Self> {
        ArrayIterator::new(self)
    }
}

/// [`ArrayBuilder`] for [`String`].
pub struct StringArrayBuilder {
    /// The flattened data of string.
    data: Vec<u8>,

    /// Offsets of each string in the data flat array.
    offsets: Vec<usize>,

    /// The null bitmap of this array.
    bitmap: BitVec,
}

impl ArrayBuilder for StringArrayBuilder {
    type Array = StringArray;

    fn with_capacity(capacity: usize) -> Self {
        let mut offsets = Vec::with_capacity(capacity + 1);
        offsets.push(0);
        Self {
            data: Vec::with_capacity(capacity),
            bitmap: BitVec::with_capacity(capacity),
            offsets,
        }
    }

    fn push(&mut self, value: Option<&str>) {
        match value {
            Some(v) => {
                self.data.extend(v.as_bytes());
                self.offsets.push(self.data.len());
                self.bitmap.push(true);
            }
            None => {
                self.offsets.push(self.data.len());
                self.bitmap.push(false);
            }
        }
    }

    fn finish(self) -> Self::Array {
        StringArray {
            data: self.data,
            bitmap: self.bitmap,
            offsets: self.offsets,
        }
    }
}