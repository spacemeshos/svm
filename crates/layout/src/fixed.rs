use crate::{Id, RawVar};

/// In-memory representation of a program's fixed-sized storage variables.
#[derive(Debug, PartialEq, Clone)]
pub struct FixedLayout {
    first: Option<Id>,

    vars: Vec<RawVar>,
}

impl Default for FixedLayout {
    fn default() -> Self {
        Self {
            first: None,
            vars: Vec::new(),
        }
    }
}

impl FixedLayout {
    pub fn new(vars: Vec<RawVar>) -> Self {
        if vars.is_empty() {
            Self::default()
        } else {
            let first = vars.get(0).map(|var| var.id);

            Self { first, vars }
        }
    }

    pub fn from_byte_sizes(first: Id, lengths: &[u32]) -> Self {
        let mut vars = vec![];
        let mut offset = 0;
        for (i, byte_size) in lengths.iter().enumerate() {
            let id = first + i as u32;
            vars.push(RawVar {
                id,
                offset,
                byte_size: *byte_size,
            });
            offset += byte_size;
        }

        Self {
            first: Some(first),
            vars,
        }
    }

    /// Returns a fixed-variable's layout
    ///
    /// # Panics
    ///
    /// Panics when there is no layout to variable `var_id`
    ///
    #[inline]
    pub fn try_get(&self, id: Id) -> Option<&RawVar> {
        let index = self.var_index(id);

        self.vars.get(index)
    }

    #[inline]
    pub fn get(&self, id: Id) -> &RawVar {
        let index = self.var_index(id);

        &self.vars[index]
    }

    /// Returns a iterator over the layout-variables.
    /// The iterators will return each time an entry of `(var_id, var_offset, var_length)`.
    pub fn iter(&self) -> LayoutIter {
        LayoutIter::new(self)
    }

    #[inline]
    pub fn try_first(&self) -> Option<Id> {
        self.first
    }

    #[inline]
    pub fn first(&self) -> Id {
        self.first.unwrap()
    }

    /// The number of variables mapped by the layout.
    #[inline]
    pub fn len(&self) -> usize {
        self.vars.len()
    }

    /// Whether layout has variables
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.vars.is_empty()
    }

    /// Returns the variable index as `usize`.
    ///
    /// # Panics
    ///
    /// Panics when `var_id` is out-of-range.
    #[inline]
    fn var_index(&self, id: Id) -> usize {
        assert!(id >= self.first());

        let index = (id - self.first()) as usize;

        assert!(index < self.vars.len());

        index
    }
}

pub struct LayoutIter<'iter> {
    offset: usize,

    current: Option<Id>,

    layout: &'iter FixedLayout,
}

impl<'iter> LayoutIter<'iter> {
    pub fn new(layout: &'iter FixedLayout) -> Self {
        Self {
            offset: 0,
            current: layout.try_first(),
            layout,
        }
    }
}

impl<'iter> std::iter::Iterator for LayoutIter<'iter> {
    type Item = RawVar;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.and_then(|current| {
            if self.offset >= self.layout.len() {
                self.current = None;

                return None;
            }

            let var = self.layout.get(current);

            self.offset += 1;
            self.current = Some(current + 1);

            Some(var.clone())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layout_new() {
        let layout = FixedLayout::from_byte_sizes(3, &[10, 20]);

        assert_eq!(layout.get(3), &RawVar::new(3, 0, 10));
        assert_eq!(layout.get(4), &RawVar::new(4, 10, 20));
    }

    #[test]
    fn layout_from_slice() {
        let layout = FixedLayout::from_byte_sizes(0, &[20, 40]);

        assert_eq!(layout.get(0), &RawVar::new(0, 0, 20));
        assert_eq!(layout.get(1), &RawVar::new(1, 20, 40));
    }

    #[test]
    fn layout_extend_from_slice() {
        let layout = FixedLayout::from_byte_sizes(4, &[10, 20, 30, 40]);

        assert_eq!(layout.get(4), &RawVar::new(4, 0, 10));
        assert_eq!(layout.get(5), &RawVar::new(5, 10, 20));
        assert_eq!(layout.get(6), &RawVar::new(6, 30, 30));
        assert_eq!(layout.get(7), &RawVar::new(7, 60, 40));
    }

    #[test]
    fn layout_iter() {
        let layout = FixedLayout::from_byte_sizes(1, &[10, 20]);

        let mut iter = layout.iter();

        let first = iter.next();
        let second = iter.next();
        let third = iter.next();
        let fourth = iter.next();

        assert_eq!(first, Some(RawVar::new(1, 0, 10)));
        assert_eq!(second, Some(RawVar::new(2, 10, 20)));

        assert_eq!(third, None);
        assert_eq!(fourth, None);
    }
}
