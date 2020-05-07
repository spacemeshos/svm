use super::DataLayoutBuilder;

/// Repersents a variable. an unsigned integer.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct VarId(pub u32);

#[derive(PartialEq, Clone)]
pub struct DataLayout {
    pub(crate) vars: Vec<(u32, u32)>,
}

impl DataLayout {
    /// For tests that don't care about the `DataLayout`
    pub fn empty() -> Self {
        Self { vars: Vec::new() }
    }

    /// Returns varialbe's layout. i.e: `(offset, length)`
    ///
    /// # Panics
    ///
    /// Panics when there is no layout to variable `var_id`
    pub fn get_var(&self, var_id: VarId) -> (u32, u32) {
        let vid = self.var_index(var_id);

        self.vars[vid]
    }

    pub fn iter(&self) -> DataLayoutIter {
        DataLayoutIter {
            cur: 0,
            layout: self,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.vars.len()
    }

    /// # Panics
    ///
    /// Panics when `var_id` is out-of-range.
    #[inline]
    fn var_index(&self, var_id: VarId) -> usize {
        let vid = var_id.0 as usize;

        assert!(vid < self.vars.capacity());

        vid
    }
}

impl From<&[u32]> for DataLayout {
    fn from(slice: &[u32]) -> Self {
        let nvars = slice.len();

        let mut builder = DataLayoutBuilder::with_capacity(nvars);
        builder.extend_from_slice(slice);

        builder.build()
    }
}

impl From<Vec<u32>> for DataLayout {
    #[inline]
    fn from(vec: Vec<u32>) -> Self {
        (*vec).into()
    }
}

pub struct DataLayoutIter<'iter> {
    cur: usize,

    layout: &'iter DataLayout,
}

impl<'iter> std::iter::Iterator for DataLayoutIter<'iter> {
    type Item = (VarId, u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur >= self.layout.len() {
            return None;
        }

        let var_id = VarId(self.cur as u32);
        let (off, len) = self.layout.get_var(var_id);

        self.cur += 1;

        Some((var_id, off, len))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_layout_new() {
        let mut builder = DataLayoutBuilder::with_capacity(2);
        builder.add_var(10);
        builder.add_var(20);

        let layout = builder.build();

        assert_eq!(layout.get_var(VarId(0)), (0, 10));
        assert_eq!(layout.get_var(VarId(1)), (10, 20));
    }

    #[test]
    fn data_layout_from_slice() {
        let vec = vec![20, 40];

        let mut layout: DataLayout = (*vec).into();

        assert_eq!(layout.get_var(VarId(0)), (0, 20));
        assert_eq!(layout.get_var(VarId(1)), (20, 40));
    }

    #[test]
    fn data_layout_extend_from_slice() {
        let mut builder = DataLayoutBuilder::with_capacity(2);
        builder.add_var(10);
        builder.add_var(20);

        builder.extend_from_slice(&[30, 40]);

        let layout = builder.build();

        assert_eq!(layout.get_var(VarId(0)), (0, 10));
        assert_eq!(layout.get_var(VarId(1)), (10, 20));
        assert_eq!(layout.get_var(VarId(2)), (30, 30));
        assert_eq!(layout.get_var(VarId(3)), (60, 40));
    }

    #[test]
    fn data_layout_iter() {
        let mut builder = DataLayoutBuilder::with_capacity(2);
        builder.add_var(10);
        builder.add_var(20);

        let layout = builder.build();

        let mut iter = layout.iter();

        let first = iter.next();
        let second = iter.next();
        let third = iter.next();
        let fourth = iter.next();

        assert_eq!(first, Some((VarId(0), 0, 10)));
        assert_eq!(second, Some((VarId(1), 10, 20)));

        assert_eq!(third, None);
        assert_eq!(fourth, None);
    }
}
