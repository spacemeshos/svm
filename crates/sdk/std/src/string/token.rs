/// Represents a Token.
pub enum Token {
    /// Token consisting of a single byte.
    One(u8),

    /// Token consisting of a pair of bytes.
    ///
    /// Examples: `=>`, `==`, `!=`
    Two(u8, u8),
}
