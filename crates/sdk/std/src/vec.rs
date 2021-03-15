use svm_sdk_alloc::alloc;

use core::fmt;
use core::mem::size_of;
use core::ops::{Deref, DerefMut};

use crate::ensure;

pub struct Vec<T> {
    len: usize,

    cap: usize,

    ptr: *mut T,
}

impl<T> Vec<T> {
    pub fn with_capacity(cap: usize) -> Self {
        let ptr = Vec::alloc(cap);

        Self { len: 0, cap, ptr }
    }

    pub fn push(&mut self, value: T) {
        ensure!(self.len() < self.capacity());

        unsafe {
            let dest = self.get_ptr_mut(self.len());

            core::ptr::write(dest, value);
        }

        self.len += 1;
    }

    pub fn pop(&mut self) -> T {
        ensure!(self.len() > 0);

        let last = self.len() - 1;

        let value = unsafe { self.take(last) };

        self.len = last;

        value
    }

    pub fn as_slice(&self) -> &[T] {
        unsafe { core::slice::from_raw_parts(self.ptr, self.len) }
    }

    pub fn as_mut(&self) -> &mut [T] {
        unsafe { core::slice::from_raw_parts_mut(self.ptr, self.len) }
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }

    pub fn leak(self) -> &'static [T] {
        let slice = self.as_slice();

        let vec = unsafe { core::mem::transmute(slice) };

        core::mem::forget(self);

        vec
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.cap
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> Iter<T> {
        Iter::new(self)
    }

    pub fn iter_mut(&mut self) -> Iter<T> {
        Iter::new(self)
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter::new(self)
    }

    unsafe fn take(&mut self, offset: usize) -> T {
        ensure!(self.len() > offset);

        let dest = self.get_ptr_mut(offset);

        core::ptr::read(dest)
    }

    fn get(&self, offset: usize) -> &T {
        ensure!(self.len() > offset);

        unsafe { self.get_unchecked(offset) }
    }

    fn get_mut(&mut self, offset: usize) -> &mut T {
        ensure!(self.len() > offset);

        unsafe { self.get_mut_unchecked(offset) }
    }

    #[inline]
    unsafe fn get_unchecked(&self, offset: usize) -> &T {
        let ptr = self.get_ptr(offset);

        &*ptr
    }

    #[inline]
    unsafe fn get_mut_unchecked(&mut self, offset: usize) -> &mut T {
        let ptr = self.get_ptr_mut(offset);

        &mut *ptr
    }

    #[inline]
    unsafe fn get_ptr(&self, offset: usize) -> *const T {
        self.ptr.offset(offset as isize)
    }

    #[inline]
    unsafe fn get_ptr_mut(&self, offset: usize) -> *mut T {
        self.ptr.offset(offset as isize)
    }

    #[inline]
    fn alloc(size: usize) -> *mut T {
        let nbytes = size_of::<T>() * size;

        alloc(nbytes).as_mut_ptr() as _
    }
}

#[cfg(feature = "debug")]
impl<T> core::fmt::Debug for Vec<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("svm_sdk::Vec")
            .field("len", &self.len())
            .field("capacity", &self.capacity())
            .finish()
    }
}

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

pub struct Iter<'a, T> {
    pos: usize,

    vec: &'a Vec<T>,
}

impl<'a, T> Iter<'a, T> {
    pub fn new(vec: &'a Vec<T>) -> Self {
        Self { vec, pos: 0 }
    }
}

impl<'a, T> core::iter::Iterator for Iter<'a, T> {
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

pub struct IntoIter<T> {
    pos: usize,

    vec: Vec<T>,
}

impl<T> IntoIter<T> {
    pub fn new(vec: Vec<T>) -> Self {
        Self { vec, pos: 0 }
    }
}

impl<T> core::iter::Iterator for IntoIter<T> {
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
