use crate::default::DefaultPageHasher;
use crate::leveldb::LevelDB;
use crate::MerklePageStorage;

use svm_common::DefaultKeyHasher;

pub type LeveldbMerklePages = MerklePageStorage<LevelDB, DefaultKeyHasher, DefaultPageHasher>;

#[cfg(test)]
mod tests {
    #[test]
    fn first_run() {
        unimplemented!()
    }
}
