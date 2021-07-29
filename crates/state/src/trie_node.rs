use svm_hash::Hasher;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TrieNode<'a> {
    Branch {
        prefix: &'a [u8],
        children_hashes: [&'a [u8]; 2],
    },
    Leaf {
        prefix: &'a [u8],
        value: &'a [u8],
    },
}

impl<'a> TrieNode<'a> {
    pub fn hash<H>(&self) -> H::Hash
    where
        H: Hasher,
    {
        // TODO: without allocations
        H::hash(&self.encode())
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut bytes = vec![];
        match self {
            Self::Branch {
                prefix,
                children_hashes: [child_l_hash, child_r_hash],
            } => {
                bytes.push(b'I');
                bytes.extend_from_slice(child_l_hash);
                bytes.extend_from_slice(child_r_hash);
                bytes.extend_from_slice(prefix);
            }
            Self::Leaf { prefix, value } => {
                bytes.push(b'L');
                bytes.push(prefix.len() as u8);
                bytes.extend_from_slice(prefix);
                bytes.extend_from_slice(value);
            }
        }
        bytes
    }

    pub fn decode(bytes: &'a [u8]) -> Option<Self> {
        if bytes.len() < 2 {
            return None;
        }
        match bytes[0] {
            b'I' => {
                if bytes.len() < 65 {
                    return None;
                }
                Some(Self::Branch {
                    prefix: &bytes[65..],
                    children_hashes: [&bytes[1..33], &bytes[33..65]],
                })
            }
            b'L' => {
                let prefix_len = bytes[1] as usize;
                Some(Self::Leaf {
                    prefix: &bytes[2..2 + prefix_len],
                    value: &bytes[2 + prefix_len..],
                })
            }
            _ => return None,
        }
    }
}
