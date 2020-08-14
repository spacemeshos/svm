//!     Execute `AppTransaction` Raw Format Version 0.0
//!  +-----------------------------------------------------+
//!  |   proto     |                                       |
//!  |  version    |            `AppAddress`               |
//!  |  encoding   |             (Address)                 |
//!  |    (a)      |                                       |
//!  |_____________|_______________________________________|
//!  |                                                     |
//!  |             Function Index Encoding (b)             |
//!  |_____________________________________________________|
//!  |            |                                        |
//!  |  calldata  |                                        |
//!  |  #length   |           calldata blob                |
//!  |  encoding  |                                        |
//!  |    (c)     |                                        |
//!  |____________|________________________________________|
//!
//!
//!
//! (a) Proto Version Encoding
//! ==========================
//!  +__________________________________________________+
//!  |       |           |                              |
//!  |  MSB  |  non-MSB  |         Meaning              |
//!  ----------------------------------------------------
//!  |   1   |  x  x  x  |  Next nibble is relevant too |
//!  |   0   |  x  x  x  |  Next nibble isn't relevant  |
//!  +--------------------------------------------------+
//!
//!  The protocol `#bits` will be a multiplication of 3,
//!  and is encoded in Big-Endian as an unsigned-integer.
//!
//!
//! (b) Function Index Encoding
//! ============================
//!
//!  Number is represented in Big-Endian layout (an unsigned-integer).
//!
//!  +---------------------------------------------------------+
//!  |  MSBs  |  Rest  |                 Meaning               |
//!  -----------------------------------------------------------
//!  |  0 0   |  .  .  |  Index takes 1 nibble  (2 used bits)  |
//!  |  0 1   |  .  .  |  Index takes 2 nibbles (6 used bits)  |
//!  |  1 0   |  .  .  |  Index takes 3 nibbles (10 used bits) |
//!  |  1 1   |  .  .  |  Index takes 4 nibbles (14 uses bits) |
//!  +--------|--------|---------------------------------------+
//!
//!  The `#bits` used numbers are `2 / 6 / 10 / 14` since the two MSB bits
//!  of the 1st nibble tell us how many nibbles are part of the encoding.
//!
//!  So if we need 4 nibbles for representing the function index,
//!  The first nibble will donate 2 bits and the other 3 nibbles will donate 4 bits each.
//!  So we get: 2 + 3 * 4 = 14
//!

mod wire;

pub use wire::{decode_exec_app, encode_exec_app};
