/// Wraps a `data` in a thread-safe manner.
#[derive(Clone, Copy)]
pub struct DataWrapper<T> {
    data: T,
}

impl<T> DataWrapper<T> {
    /// Receives `data` to wrap
    pub fn new(data: T) -> Self {
        Self { data }
    }

    /// Releases `self` and returns its wrapped `data`
    pub fn unwrap(self) -> T {
        self.data
    }
}

unsafe impl<T> Sync for DataWrapper<T> {}
unsafe impl<T> Send for DataWrapper<T> {}
