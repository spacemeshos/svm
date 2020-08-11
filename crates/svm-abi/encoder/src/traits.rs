use svm_nibble::NibbleWriter;

/// A trait used to encoding a value (of `primitive` or `composite` type)
pub trait Encoder {
    /// Encodes `self` and outputs the data into `buf`
    fn encode(&self, write: &mut NibbleWriter);
}
