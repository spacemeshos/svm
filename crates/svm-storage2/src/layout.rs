/// Repersents a variable. an unsigned integer.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct VarId(pub u32);

/// Specifies the fixed-sized variables of an application.
#[derive(PartialEq, Clone)]
pub struct DataLayout {
    vars: Vec<Option<(u32, u32)>>,
}

/// `DataLayout` represents the fixed-sized variables (storage) of an application.
impl DataLayout {
    /// New instance, initialized with the total number of variables.
    pub fn new(nvars: u32) -> Self {
        Self {
            vars: vec![None; nvars as usize],
        }
    }

    /// Adds a new variable's layout
    pub fn add_var(&mut self, var_id: VarId, offset: u32, len: u32) {
        let vid = self.var_index(var_id);

        self.vars[vid] = Some((offset, len));
    }

    /// Returns varialbe's layout. i.e: `(offset, length)`
    ///
    /// # Panics
    ///
    /// Panics when there is no layout to variable `var_id`
    pub fn get_var(&self, var_id: VarId) -> (u32, u32) {
        let vid = self.var_index(var_id);

        self.vars[vid].unwrap()
    }

    #[inline]
    pub fn len(&self) -> u32 {
        self.vars.len() as u32
    }

    pub fn iter(&self) -> DataLayoutIter {
        DataLayoutIter {
            cur: 0,
            layout: self,
        }
    }

    ///
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

pub struct DataLayoutIter<'iter> {
    cur: u32,

    layout: &'iter DataLayout,
}

impl<'iter> std::iter::Iterator for DataLayoutIter<'iter> {
    type Item = (VarId, u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur >= self.layout.len() {
            return None;
        }

        let var_id = VarId(self.cur);
        let (off, len) = self.layout.get_var(var_id);

        self.cur += 1;

        Some((var_id, off, len))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_layout_sanity() {
        let mut layout = DataLayout::new(2);

        layout.add_var(VarId(0), 10, 20);
        layout.add_var(VarId(1), 30, 40);

        assert_eq!(layout.get_var(VarId(0)), (10, 20));
        assert_eq!(layout.get_var(VarId(1)), (30, 40));

        let mut iter = layout.iter();

        let first = iter.next();
        let second = iter.next();
        let third = iter.next();
        let fourth = iter.next();

        assert_eq!(first, Some((VarId(0), 10, 20)));
        assert_eq!(second, Some((VarId(1), 30, 40)));

        assert_eq!(third, None);
        assert_eq!(fourth, None);
    }
}
