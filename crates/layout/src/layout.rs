use crate::{Id, LayoutBuilder, RawVar};

/// In-memory representation of a program's fixed-sized storage variables.
#[derive(Debug, PartialEq, Clone)]
pub struct Layout {
    pub(crate) vars: Vec<(u32, u32)>,
}

impl Layout {
    /// For tests that don't care about the `Layout`
    pub fn empty() -> Self {
        Self { vars: Vec::new() }
    }

    /// Returns variable's layout. i.e: `(offset, length)`
    ///
    /// # Panics
    ///
    /// Panics when there is no layout to variable `var_id`
    pub fn get_var(&self, id: Id) -> RawVar {
        let index = self.var_index(id);
        let (offset, byte_size) = self.vars[index];

        RawVar::new(id, offset, byte_size)
    }

    /// Returns a iterator over the layout-variables.
    /// The iterators will return each time an entry of `(var_id, var_offset, var_length)`.
    pub fn iter(&self) -> LayoutIter {
        LayoutIter {
            current: 0,
            layout: self,
        }
    }

    /// The number of variables mapped by the layout.
    #[inline]
    pub fn len(&self) -> usize {
        self.vars.len()
    }

    /// Whether layout has variables
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the variable index as `usize`.
    ///
    /// # Panics
    ///
    /// Panics when `var_id` is out-of-range.
    #[inline]
    fn var_index(&self, id: Id) -> usize {
        let id = id.0 as usize;

        assert!(id < self.vars.capacity());

        id
    }
}

impl From<&[u32]> for Layout {
    fn from(slice: &[u32]) -> Self {
        let nvars = slice.len();

        let mut builder = LayoutBuilder::with_capacity(nvars);
        builder.extend_from_slice(slice);

        builder.build()
    }
}

impl From<Vec<u32>> for Layout {
    #[inline]
    fn from(vec: Vec<u32>) -> Self {
        (*vec).into()
    }
}

pub struct LayoutIter<'iter> {
    current: usize,

    layout: &'iter Layout,
}

impl<'iter> std::iter::Iterator for LayoutIter<'iter> {
    type Item = RawVar;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.layout.len() {
            return None;
        }

        let id = Id(self.current as u32);
        let var = self.layout.get_var(id);

        self.current += 1;

        Some(var)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layout_new() {
        let mut builder = LayoutBuilder::with_capacity(2);
        builder.add_var(10);
        builder.add_var(20);

        let layout = builder.build();

        assert_eq!(layout.get_var(Id(0)), RawVar::new(Id(0), 0, 10));
        assert_eq!(layout.get_var(Id(1)), RawVar::new(Id(1), 10, 20));
    }

    #[test]
    fn layout_from_slice() {
        let vec = vec![20, 40];

        let layout: Layout = (*vec).into();

        assert_eq!(layout.get_var(Id(0)), RawVar::new(Id(0), 0, 20));
        assert_eq!(layout.get_var(Id(1)), RawVar::new(Id(1), 20, 40));
    }

    #[test]
    fn layout_extend_from_slice() {
        let mut builder = LayoutBuilder::with_capacity(2);
        builder.add_var(10);
        builder.add_var(20);

        builder.extend_from_slice(&[30, 40]);

        let layout = builder.build();

        assert_eq!(layout.get_var(Id(0)), RawVar::new(Id(0), 0, 10));
        assert_eq!(layout.get_var(Id(1)), RawVar::new(Id(1), 10, 20));
        assert_eq!(layout.get_var(Id(2)), RawVar::new(Id(2), 30, 30));
        assert_eq!(layout.get_var(Id(3)), RawVar::new(Id(3), 60, 40));
    }

    #[test]
    fn layout_iter() {
        let mut builder = LayoutBuilder::with_capacity(2);
        builder.add_var(10);
        builder.add_var(20);

        let layout = builder.build();

        let mut iter = layout.iter();

        let first = iter.next();
        let second = iter.next();
        let third = iter.next();
        let fourth = iter.next();

        assert_eq!(first, Some(RawVar::new(Id(0), 0, 10)));
        assert_eq!(second, Some(RawVar::new(Id(1), 10, 20)));

        assert_eq!(third, None);
        assert_eq!(fourth, None);
    }
}
