/// This file implements a `Vec` of a fixed-size.
/// In order to use it, the maximum number of elements should be known before calling `Vec#with_capacity.
/// The underlying allocated memory won't resize (grow or shrink) nor will it move.
///
/// The API of the fixed-sized `Vec` is a subset of `std::vec::Vec`.
/// So in case we want to swap the implementation such that `svm_sdk_std::Vec` will be `std::vec::Vec`
/// just comment the code of the file and add these lines instead
///
/// ```rust
/// extern crate alloc;
/// pub use alloc::vec::Vec;
/// ```
///
/// This can be useful when debugging. We can turn-off the fixed-size `Vec` in case we suspect it causes issues.
extern crate svm_sdk_alloc;

use svm_sdk_alloc::alloc;

use core::mem::size_of;
use core::ops::{Deref, DerefMut};

use crate::ensure;

/// Fixed-Gas replacement for
/// [`std::vec::Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html).
pub struct Vec<T> {
    len: usize,
    cap: usize,
    ptr: *mut T,
}

impl<T> Vec<T> {
    /// Allocating a fixed-size `Vec`.  More info above.
    pub fn with_capacity(cap: usize) -> Self {
        let ptr = Vec::alloc(cap);

        Self { len: 0, cap, ptr }
    }

    /// Initializes a new [`Vec`] given a raw pointer to the first item and the number of items.
    pub unsafe fn from_raw_parts(ptr: *const T, len: usize) -> Self {
        Self {
            len,
            cap: len,
            ptr: ptr as *mut T,
        }
    }

    /// Appends a new item.
    pub fn push(&mut self, value: T) {
        ensure!(self.len() < self.capacity());

        unsafe {
            let dest = self.as_ptr_mut(self.len());
            core::ptr::write(dest, value);
        }

        self.len += 1;
    }

    /// Pops the last pushed item and returns it.
    ///
    /// # Panics
    ///
    /// Panics if the `self` is empty.
    pub fn pop(&mut self) -> T {
        ensure!(self.is_empty() == false);

        let last = self.len() - 1;
        let value = unsafe { self.take(last) };
        self.len = last;

        value
    }

    /// Returns a shared view over the underlying items.
    pub fn as_slice(&self) -> &[T] {
        unsafe { core::slice::from_raw_parts(self.ptr, self.len) }
    }

    /// Returns a mutable view over the underlying items.
    pub fn as_mut(&mut self) -> &mut [T] {
        unsafe { core::slice::from_raw_parts_mut(self.ptr, self.len) }
    }

    /// Clears the `self`, turns it into an empty [`Vec`].
    pub fn clear(&mut self) {
        self.len = 0;
    }

    /// Leaks the underlying items and returns a slice to it with a `static` lifetime.
    pub fn leak(self) -> &'static [T] {
        let slice = self.as_slice();

        let vec = unsafe { core::mem::transmute(slice) };
        core::mem::forget(self);

        vec
    }

    /// Returns the number of taken items.
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns the number of items that `self` can hold.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.cap
    }

    /// Returns whether `self` is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns an iterator over the contained items.
    pub fn iter(&self) -> VecIter<T> {
        VecIter { pos: 0, vec: self }
    }

    /// Returns a mutable iterator over the contained items.
    pub fn iter_mut(&mut self) -> VecIter<T> {
        VecIter { pos: 0, vec: self }
    }

    /// Transfers ownership of self and returns a new [`VecIntoIter`].
    pub fn into_iter(self) -> VecIntoIter<T> {
        VecIntoIter { vec: self, pos: 0 }
    }

    unsafe fn take(&mut self, offset: usize) -> T {
        ensure!(self.len() > offset);

        let dest = self.as_ptr_mut(offset);
        core::ptr::read(dest)
    }

    fn get(&self, offset: usize) -> &T {
        ensure!(self.len() > offset);

        unsafe { self.get_unchecked(offset) }
    }

    #[inline]
    unsafe fn get_unchecked(&self, offset: usize) -> &T {
        let ptr = self.as_ptr(offset);

        &*ptr
    }

    #[inline]
    unsafe fn as_ptr(&self, offset: usize) -> *const T {
        self.ptr.add(offset)
    }

    #[inline]
    unsafe fn as_ptr_mut(&self, offset: usize) -> *mut T {
        self.ptr.add(offset)
    }

    #[inline]
    fn alloc(size: usize) -> *mut T {
        let nbytes = size_of::<T>() * size;

        alloc(nbytes).as_mut_ptr() as _
    }
}

#[cfg(any(test, feature = "debug"))]
impl<T> core::fmt::Debug for Vec<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("svm_sdk::Vec")
            .field("len", &self.len())
            .field("capacity", &self.capacity())
            .finish()
    }
}

#[cfg(any(test, feature = "debug"))]
impl<T: PartialEq> PartialEq for Vec<T> {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<T> Deref for Vec<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T> DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

pub struct VecIter<'a, T> {
    pos: usize,
    vec: &'a Vec<T>,
}

impl<'a, T> core::iter::Iterator for VecIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.vec.len() {
            return None;
        }

        let item = self.vec.get(self.pos);
        self.pos += 1;

        Some(item)
    }
}

/// An iterator for [`Vec<T>`].
pub struct VecIntoIter<T> {
    pos: usize,
    vec: Vec<T>,
}

impl<T> core::iter::Iterator for VecIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.vec.len() {
            return None;
        }

        let v = unsafe { self.vec.take(self.pos) };
        self.pos += 1;

        Some(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_empty() {
        let vec: Vec<u8> = Vec::with_capacity(0);

        assert!(vec.is_empty());
        assert_eq!(vec.len(), 0);
    }

    #[test]
    fn vec_one_item() {
        let mut vec: Vec<u8> = Vec::with_capacity(1);

        assert!(vec.is_empty());
        vec.push(10);
        assert_eq!(vec.len(), 1);
        assert!(vec.is_empty() == false);

        let v = vec.pop();
        assert_eq!(v, 10);
        assert!(vec.is_empty());
    }

    #[test]
    fn vec_two_items() {
        let mut vec: Vec<u8> = Vec::with_capacity(2);

        assert!(vec.is_empty());
        vec.push(10);
        vec.push(20);

        assert_eq!(vec.len(), 2);

        let v = vec.pop();
        assert_eq!(v, 20);

        let v = vec.pop();
        assert_eq!(v, 10);

        assert!(vec.is_empty());
    }

    #[should_panic]
    #[test]
    fn vec_pop_from_empty_panics() {
        let mut vec: Vec<u8> = Vec::with_capacity(1);

        assert!(vec.is_empty());
        vec.push(10);

        let _ = vec.pop();
        let _ = vec.pop();
    }

    #[test]
    fn vec_clear() {
        let mut vec: Vec<u8> = Vec::with_capacity(2);

        vec.push(10);
        vec.push(20);

        assert_eq!(vec.len(), 2);

        vec.clear();

        assert!(vec.is_empty());
    }

    #[test]
    fn vec_leak() {
        let mut vec: Vec<u8> = Vec::with_capacity(2);

        vec.push(10);
        vec.push(20);

        let slice: &'static [u8] = vec.leak();

        assert_eq!(slice, &[10, 20]);
    }

    #[test]
    fn vec_iter() {
        let mut vec: Vec<u8> = Vec::with_capacity(2);

        vec.push(10);
        vec.push(20);

        let mut iter = vec.iter();

        let v = iter.next().unwrap();
        assert_eq!(v, &10);

        let v = iter.next().unwrap();
        assert_eq!(v, &20);

        assert_eq!(iter.next(), Option::None);
    }

    #[test]
    fn vec_into_iter() {
        let mut vec: Vec<u8> = Vec::with_capacity(2);

        vec.push(10);
        vec.push(20);

        let mut iter = vec.into_iter();

        let v = iter.next().unwrap();
        assert_eq!(v, 10);

        let v = iter.next().unwrap();
        assert_eq!(v, 20);

        assert_eq!(iter.next(), Option::None);
    }
}
